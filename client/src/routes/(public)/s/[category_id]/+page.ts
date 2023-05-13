import { PUBLIC_API_URL } from "$env/static/public";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";
import type { Category, Post } from "$lib/util/types";

export const load: PageLoad = async ({fetch, params, url}) => {

    const show_all = url.searchParams.has("show_all");
    
    const getCategory = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/category/" + params.category_id);
            const data = await response.json();
            return data.category as Category;

        } catch (_) {
            throw error(404, "Not Found");
        }
    }

    const getPosts = async () => {
        try {
            const response = await fetch(PUBLIC_API_URL + "/api/category/" + params.category_id + "/posts" + "?show_all=" + show_all);
            const data = await response.json();
            return data.posts as Post[];

        } catch (_) {
            return [];
        }
    }

    return {
        category: getCategory(),
        posts: getPosts()
    }
};