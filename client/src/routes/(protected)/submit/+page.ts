import type { PageLoad } from "./$types";
import { PUBLIC_API_URL } from "$env/static/public";
import type { Category, Post } from "$lib/util/types";
import { error } from "@sveltejs/kit";

export const load: PageLoad = async ({fetch, url}) => {

    let path = '/api/category';
    let subtidder = false;

    const getCategories = async () => {
        
        if (url.searchParams.has("subtidder")) {
            path += `/${url.searchParams.get("subtidder")}`;
            subtidder = true;
        }
        
        const response = await fetch(PUBLIC_API_URL + path);
        
        if (!response.ok) throw error(404, "Not Found");

        const data = await response.json();
        
        return subtidder 
            ? [data.category] as Category[]
            : data.categories as Category[];
    }

    return {
        categories: getCategories(),
    }
    
};