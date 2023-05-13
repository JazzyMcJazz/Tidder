import { PUBLIC_API_URL } from "$env/static/public";
import type { Category, Post } from "$lib/util/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({fetch, url}) => {    

    const show_all = url.searchParams.has("show_all");

    const getPopularPosts = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/post/popular" + "?show_all=" + show_all);
            const data = await response.json();
            return data.posts as Post[];
            
        } catch (_) {
            return [];
        }
    }

    const getCategories = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/category");
            const data = await response.json();
            return data.categories as Category[];
            
        } catch (_) {
            return [];
        }
    }

    return {
        posts: getPopularPosts(),
        categories: getCategories()
    }
    
};