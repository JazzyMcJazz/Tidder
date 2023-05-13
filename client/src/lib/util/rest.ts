import { PUBLIC_API_URL } from "$env/static/public";
import type { Category, Post } from "$lib/util/types";

interface CreatePostForm {
    title: string,
    body: string,
    category_id?: string,
    new_category?: string,
    draft?: boolean,
}

class REST {

    async getPubkey() {
        const response = await fetch(PUBLIC_API_URL + "/api/pubkey");
        return await response.text();
    }

    async register(username: string, password: string) {
        return await fetch(PUBLIC_API_URL + '/api/register', {
            method: 'POST',
            credentials: 'include',
            headers: { 
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: `username=${username}&password=${password}`
        });
    }
    
    async login(username: string, password: string) {
        return await fetch(PUBLIC_API_URL + '/api/login', {
            method: 'POST',
            credentials: 'include',
            headers: { 
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: `username=${username}&password=${password}`
        });
    }

    async logout() {
        return await fetch(PUBLIC_API_URL + '/api/logout', {
            method: 'POST',
            credentials: 'include',
        });
    }

    async createPost({ title, body, category_id, new_category, draft }: CreatePostForm) {
        let bodyString = `title=${title}&body=${body}`;
        if (category_id) {
            bodyString += `&category_id=${category_id}`;
        } else if (new_category) {
            bodyString += `&new_category=${new_category}`;
        }
        
        return await fetch(PUBLIC_API_URL + '/api/post' + (draft ? '?draft=true' : ''), {
            method: 'POST',
            credentials: 'include',
            headers: { 
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: bodyString,
        });
    }

    async publishPost(post_id: string) {
        return await fetch(PUBLIC_API_URL + '/api/post/' + post_id + '/publish', {
            method: 'POST',
            credentials: 'include',
        });
    }
    
    async deletePost(post_id: string) {
        return await fetch(PUBLIC_API_URL + '/api/post/' + post_id, {
            method: 'DELETE',
            credentials: 'include',
        });
    }

    async createComment(post_id: string, body: string) {
        return await fetch(PUBLIC_API_URL + '/api/post/' + post_id + '/comment', {
            method: 'POST',
            credentials: 'include',
            headers: { 
                'Content-Type': 'application/x-www-form-urlencoded',
            },
            body: `body=${body}`,
        });
    }

    async deleteComment(comment_id: string) {
        return await fetch(PUBLIC_API_URL + '/api/comment/' + comment_id, {
            method: 'DELETE',
            credentials: 'include',
        });
    }

    async search(query: string) {
        const response = await fetch(PUBLIC_API_URL + '/api/search?q=' + query);
        const data = await response.json();
        return {
            categories: data.categories as Category[],
            posts: data.posts as Post[],
        }
    }

    async uploadAvatar(file: File) {
        const formData = new FormData();
        formData.append('file', file);
        
        return await fetch(PUBLIC_API_URL + '/api/upload/avatar', {
            method: 'POST',
            credentials: 'include',
            body: formData,
        });
    }
}

export default new REST();