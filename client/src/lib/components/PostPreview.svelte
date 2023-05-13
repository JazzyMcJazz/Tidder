<script lang="ts">
	import { page } from "$app/stores";
	import type { Post } from "$lib/util/types";

    export let data: Post;
</script>

<a href="/s/{data.category_id}/{data.id}">
    <div class="post bg-primary hover:brightness-125">
        <div class="flex">
            {#if $page.url.searchParams.has('show_all') && $page.data.user?.role === 'admin'}
                {#if data.deleted}    
                    <h3 class="post-title text-red-500 mr-1">[deleted]</h3>
                {/if}
                {#if !data.published}
                    <h3 class="post-title text-accent mr-1">[draft]</h3>
                {/if}
            {/if}
            <h3 class="post-title">{@html data.title}</h3>
        </div>
        <p class="post-body">{@html data.body.length > 50 ? data.body.slice(0, 50) + '...' : data.body}</p>
        <p class="post-date">{new Date(data.created_at).toLocaleString()}</p>
        <p class="post-author">Posted by: {data.author_name}</p>
    </div>
</a>