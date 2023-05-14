<script lang="ts">
    import type { Comment } from '$lib/util/types';
	import { PUBLIC_API_URL } from '$env/static/public';
    import { page } from '$app/stores';
    import ArrowDown from 'svelte-icons/fa/FaArrowDown.svelte';
    import ArrowUp from 'svelte-icons/fa/FaArrowUp.svelte';
    import REST from '$lib/util/rest';
	import { invalidateAll } from '$app/navigation';

    export let data: Comment;
    export let avatar_url: string | undefined;

    let loading = false;

    $: user = $page.data.user;

    const handleDeleteComment = async () => {
        loading = true;

        const response = await REST.deleteComment(data.id);
        
        if (response.ok) {
            setTimeout(async () => {
                await invalidateAll();
                loading = false;
            }, 1000);
        } else {
            loading = false;
        }
    }
</script>

<div class="comment flex mb-4">
    <div class="mr-4 mt-1 flex flex-col items-center">
        <button type="button" class="w-5 px-1 mb-1 text-tertiary hover:text-accent">
            <ArrowUp/>
        </button>
        <p class="text-light font-semibold">{data.upvotes - data.downvotes}</p>
        <button type="button" class="w-5 px-1 mt-1 text-tertiary hover:text-red-500">
            <ArrowDown/>
        </button>
    </div>
    <div class="w-full">
        <div class="flex justify-between items-center">
            <div class="flex items-center">
                <img
                    src="{avatar_url || '/fallback.png'}"
                    alt="avatar"
                    class="w-10 h-10 rounded-full mr-2 object-cover border-2 border-accent"
                />
                <p class="comment-author">{data.author_name}</p>
                <p class="comment-date">{new Date(data.created_at).toLocaleString()}</p>
            </div>
            
            <!-- Delete Button -->
            {#if !data.deleted && user && (user.id === data.author_id || user.role === 'admin')}
                <form
                    method="delete"
                    action="{PUBLIC_API_URL}/api/comments/{data.id}"
                    on:submit|preventDefault={handleDeleteComment}
                >
                    <button class="text-sm text-red-500 hover:underline">
                        {#if loading}
                            <div class="w-6 h-6 mx-auto border-t-2 border-red-500 rounded-full animate-spin"></div>
                        {:else}
                            Delete
                        {/if}
                    </button>
                </form>
            {/if}
        </div>
        {#if data.deleted && $page.data.user.role === 'admin' && $page.url.searchParams.has('show_all')}
            <p class="text-red-500">[comment deleted]</p>    
        {/if}
        <p class="comment-body">{@html data.body}</p>
    </div>
</div>