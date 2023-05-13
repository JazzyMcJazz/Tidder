import { PUBLIC_API_URL } from '$env/static/public';
import type { PageLoad } from './$types';

export const load = (async ({fetch, parent}) => {

    const user = (await parent()).user;

    const fetchAvatarUrl = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/avatar?user_ids=" + user.id);
            
            const data = await response.json();
            return data.urls[0].avatar_url;
        } catch (_) {
            return null;
        }
    }

    const fetchPosts = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/post/me", {
                credentials: "include"
            });

            const data = await response.json();
            return data.posts;
        } catch (_) {
            return [];
        }
    }

    return {
        avatar_url: fetchAvatarUrl(),
        posts: fetchPosts()
    };
}) satisfies PageLoad;