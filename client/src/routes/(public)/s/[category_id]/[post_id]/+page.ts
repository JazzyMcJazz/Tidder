import type { Category, Post, Comment } from '$lib/util/types';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { PUBLIC_API_URL } from '$env/static/public';

export const load = (async ({fetch, params, url}) => {

    const show_all = url.searchParams.has("show_all");

    const fetchPost = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/post/" + params.post_id + "?show_all=" + show_all, {
                credentials: "include"
            });
            
            if (!response.ok) throw error(404, "Post not found");
            const data = await response.json();
            return data as {
                category: Category,
                post: Post
            };

        } catch (_) {
            throw error(404, "Post not found");
        }
    }

    const fetchComments = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/post/" + params.post_id + "/comment" + "?show_all=" + show_all);
            const data = await response.json();
            return data.comments as Comment[];
        } catch (_) {
            return [];
        }
    }

    const comments = await fetchComments();
    const fetchAvatarUrls = async () => {
        const user_ids_set = new Set(comments.map(comment => comment.author_id));
        const user_ids = Array.from(user_ids_set).join(",");
        
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/avatar?user_ids=" + user_ids);
            const data = await response.json();
            let avatars: { [key:string]: { avatar_url?: string } } = {}
            for (const avatar of data.urls) {
                avatars[avatar.user_id] = { avatar_url: avatar.avatar_url };
            }
            return avatars;
        } catch (_) {
            return null;
        }
    }


    return {
        postData: fetchPost(),
        comments: comments,
        avatar_urls: fetchAvatarUrls()
    };
}) satisfies PageLoad;