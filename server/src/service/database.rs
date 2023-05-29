use std::env;

use actix_web::web::block;
use r2d2::Pool;
use r2d2_sqlite::rusqlite::ToSql;
use r2d2_sqlite::{rusqlite::params, SqliteConnectionManager};
use uuid;

use crate::DbPool;
use crate::model::data::User;


pub async fn init(pool: Pool<SqliteConnectionManager>) -> Result<(), String> {
    let result = block(move || {
        let conn = pool.get()
            .expect("couldn't get db connection from pool");

        // Create tables
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                username_lower TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                avatar_url TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                role TEXT NOT NULL DEFAULT 'user'
            )", 
            params![]
        ) {
            Ok(_) => {
                println!("Database initialized");
            },
            Err(e) => {
                eprintln!("Error initializing database: {}", e);
                return Err(e);
            },
        }

        // Create default admin user
        let id = uuid::Uuid::new_v4().to_string();
        let username = env::var("ADMIN_USER").expect("ADMIN_USER not set");
        let password = env::var("ADMIN_PASS").expect("ADMIN_PASS not set");
        let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password");

        match conn.execute(
            "INSERT INTO users (id, username, username_lower, password, role) VALUES (?, ?, ?, ?, ?)", 
            params![id, username, username.to_lowercase(), password, "admin"]
        ) {
            Ok(_) => {
                println!("Default admin account created");
                Ok(())
            },
            Err(e) => {
                if e.to_string().eq("UNIQUE constraint failed: users.username") || e.to_string().eq("UNIQUE constraint failed: users.username_lower") {
                    Ok(())
                } else {
                    eprintln!("Error creating default admin user: {}", e);
                    Err(e)
                }
            },
        }

    }).await.map_err(|e| {
        panic!("Error initializing database: {}", e)
    });

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn save_user(pool: DbPool, username: String, password: String) -> Result<String, String> {
    
    let result = block(move || {
        let conn = pool.get()
            .expect("couldn't get db connection from pool");

        let id = uuid::Uuid::new_v4().to_string();

        match conn.execute(
            "INSERT INTO users (id, username, username_lower, password) VALUES (?, ?, ?, ?)", 
            params![id, username, username.to_lowercase(), password]
        ) {
            Ok(_) => Ok(id),
            Err(e) => Err(e)
        }

    }).await.map_err(|e| {
        eprintln!("{}", e);
        "Error saving user".to_string()
    }).unwrap();

    match result {
        Ok(id) => Ok(id),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn find_users(pool: DbPool) -> Result<Vec<User>, String> {
        
        let result = block(move || {
            let conn = pool.get()
                .expect("couldn't get db connection from pool");
    
            let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
            let user_iter = stmt.query_map(params![], |row| {
                Ok(User::from_db(row))
            }).unwrap();
    
            let mut users = Vec::new();
            for user in user_iter {
                users.push(user.unwrap());
            }
    
            Ok::<Vec<User>, String>(users)
    
        }).await.map_err(|e| {
            eprintln!("{}", e);
            "Error finding users".to_string()
        });
    
        match result {
            Ok(result) => match result {
                Ok(users) => Ok(users),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e),
        }
}

pub async fn find_user_by_id(pool: DbPool, id: String) -> Result<User, String> {
    
        // Open a connection on a separate thread and return the result to the main thread
        let result = block(move || {
            let conn = pool.get()
                .expect("couldn't get db connection from pool");
    
            // Query for the user
            conn.query_row(
                "SELECT * FROM users WHERE id = ?", 
                params![id],
                |row| {
                    Ok(User::from_db(row))
                }
            )
        // Handle any errors that may occur
        }).await.map_err(|e| {
            eprintln!("{}", e);
            "Error finding user".to_string()
        });
    
        match result {
            Ok(result) => match result {
                Ok(user) => Ok(user),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e),
        }

}

pub async fn find_user_by_username(pool: DbPool, username: String) -> Result<User, String> {

    let result = block(move || {
        let conn = pool.get()
            .expect("couldn't get db connection from pool");

        conn.query_row(
            "SELECT * FROM users WHERE username_lower = ?", 
            params![username.to_lowercase()],
            |row| {
                Ok(User::from_db(row))
            }
        )

    }).await.map_err(|e| {
        eprintln!("{}", e);
        "Error finding user".to_string()
    });

    match result {
        Ok(result) => match result {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e),
    }
}

pub async fn find_avatars_by_user_ids(pool: DbPool, ids: Vec<String>) -> Result<Vec<(String, Option<String>)>, String> {
    
        let result = block(move || {
            let conn = pool.get()
                .expect("couldn't get db connection from pool");
    
            let params: Vec<&dyn ToSql> = ids.iter().map(|x| x as &dyn ToSql).collect();
            let query = format!(
                "SELECT * FROM users WHERE id IN ({})",
                ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
            );

            let mut stmt = conn.prepare(&query).unwrap();
            
            let url_iter = stmt.query_map(params.as_slice(), |row| {
                let user = User::from_db(row);
                Ok((user.id, user.avatar_url))
            }).unwrap();
    
            let mut urls = Vec::new();
            for url in url_iter {
                match url {
                    Ok(url) => urls.push(url),
                    Err(e) => return Err(e.to_string()),
                }
            }
    
            Ok::<Vec<(String, Option<String>)>, String>(urls)
    
        }).await.map_err(|e| {
            eprintln!("{}", e);
            "Error finding users".to_string()
        });
    
        match result {
            Ok(result) => match result {
                Ok(urls) => Ok(urls),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e),
        }
}

pub async fn update_user_avatar(pool: DbPool, user_id: String, avatar_url: String) -> Result<(), String> {
    
    let result = block(move || {
        let conn = pool.get()
            .expect("couldn't get db connection from pool");

        match conn.execute(
            "UPDATE users SET avatar_url = ? WHERE id = ?", 
            params![avatar_url, user_id]
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }

    }).await.map_err(|e| {
        eprintln!("{}", e);
        "Error updating avatar url".to_string()
    }).unwrap();

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}