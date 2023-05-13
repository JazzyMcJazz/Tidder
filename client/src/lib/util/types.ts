export type User = {
    id: string,
    username: string,
    role: string,
}

export type Post = {
    id: string,
    author_id: number,
    author_name: string,
    category_id: number,
    category_name: string,
    title: string,
    body: string,
    upvotes: number,
    downvotes: number,
    published: boolean,
    deleted: boolean,
    created_at: string,
}

export type Category = {
    id: number,
    name: string,
    posts: number,
}

export type Comment = {
    id: string,
    author_name: string,
    author_id: string,
    post_id: string,
    body: string,
    deleted: boolean,
    upvotes: number,
    downvotes: number,
    created_at: string,
    updated_at: string,
}