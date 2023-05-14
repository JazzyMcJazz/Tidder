<script lang="ts">
    import { page } from '$app/stores';
    import ArrowUp from 'svelte-icons/fa/FaArrowUp.svelte';
    import ArrowDown from 'svelte-icons/fa/FaArrowDown.svelte';
    import REST from '$lib/util/rest';
	import { invalidateAll } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';
	import Comment from '$lib/components/Comment.svelte';
	import ShowAllButton from '$lib/components/ShowAllButton.svelte';

    $: postData = $page.data.postData; 
    $: comments = $page.data.comments;
    $: user = $page.data.user;
    $: avatar_urls = $page.data.avatar_urls;

    let form: HTMLFormElement;
    let loadingCommentSubmit = false;
    let loadingDelete = false;
    let error = '';
    let commentBody = '';

    let showCreateComment = !!$page.data.user;
    $: switch (!!$page.data.user) {
        case true:
            setTimeout(() => {
                showCreateComment = true;
            }, 500);
            break;
        default:
            showCreateComment = false;
            break;
    }

    const handleDeletePost = async () => {
        loadingDelete = true;
        error = '';

        const response = await REST.deletePost(postData.post.id);
        
        if (response.ok) {
            invalidateAll();
            
        } else {
            const data = await response.json();
            error = data.message
                .replace("Category", "Subtidder")
                .replace("category", "subtidder");

            setTimeout(() => {
                error = '';
            }, 5000);
        }
        
        loadingDelete = false;
    }

    const handleSubmitComment = async () => {
        loadingCommentSubmit = true;

        loadingCommentSubmit = true;
        error = '';

        const data = new FormData(form);
        const body = data.get('body') as string;

        if (!body || body.length === 0) {
            loadingCommentSubmit = false;
            return;
        }

        const response = await REST.createComment(postData.post.id, body);
        
        if (response.ok) {
            commentBody = '';
            invalidateAll();
            
        } else {
            const data = await response.json();
            error = data.message
                .replace("Category", "Subtidder")
                .replace("category", "subtidder");

            setTimeout(() => {
                error = '';
            }, 5000);
        }
        
        loadingCommentSubmit = false;
    }
</script>

{#if error}
    <div class="error absolute top-32 right-20 p-2 w-fit font-semibold text-center rounded-lg opacity-90 bg-red-500 z-50" >
        {error}
    </div>
{/if}

<!-- Navigation -->
<section>
    <!-- Go back to category -->
    <a href="/s/{postData.category.id}" class="flex w-fit items-center text-tertiary hover:text-accent">
        <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" 
                d="M10.707 3.293a1 1 0 010 1.414L7.414 9H16a1 1 0 
                010 2H7.414l3.293 3.293a1 1 0 01-1.414 
                1.414l-5-5a1 1 0 010-1.414l5-5a1 1 0 
                011.414 0z" 
                clip-rule="evenodd" 
            />
        </svg>
        <p class="ml-2">Go back to {postData.category.name}</p>
    </a>

</section>
  
<!-- Post -->
<section>
    <div class="flex">
        <div class="mr-4 mt-1 flex flex-col items-center">
            <button type="button" class="w-7 px-1 mb-1 text-tertiary hover:text-accent">
                <ArrowUp/>
            </button>
            <p class="text-light font-semibold">{postData.post.upvotes - postData.post.downvotes}</p>
            <button type="button" class="w-7 px-1 mt-1 text-tertiary hover:text-red-500">
                <ArrowDown/>
            </button>
        </div>

        <div class="w-full">
            <div class="flex justify-between items-center">
                <div class="flex">
                    {#if postData.post.deleted && $page.url.searchParams.has('show_all')}
                        <p class="text-2xl mr-1 text-red-500">[deleted]</p>
                    {:else if !postData.post.published}
                        <p class="text-2xl mr-1 text-accent">[draft]</p>
                    {/if}
                    <h1 class="text-2xl">{@html postData.post.title}</h1>
                </div>

                <!-- Delete button -->
                {#if !postData.post.deleted && user && (user.id === postData.post.author_id || user.role === 'admin')}
                    <form 
                        method="delete" 
                        action="{PUBLIC_API_URL}/api/post/{postData.post.id}" 
                        on:submit|preventDefault={handleDeletePost}
                    >
                        <button class="text-sm text-red-500 hover:underline">
                            {#if loadingDelete}
                                <div class="w-6 h-6 mx-auto border-t-2 border-red-500 rounded-full animate-spin"></div>
                            {:else}
                                Delete
                            {/if}
                        </button>
                    </form>
                {:else if !postData.post.published}
                    <ShowAllButton />
                {/if}
            </div>
            <div class="flex text-sm items-center mb-2">
                <p class="mr-1">In</p>
                <p class="font-semibold">{postData.category.name}</p>
                <p class="post-date mx-2">•</p>
                <p class="font-light">Posted by</p>
                <p class="post-author ml-1">{postData.post.author_name}</p>
                <p class="post-date mx-2">•</p>
                <p class="post-date">{new Date(postData.post.created_at).toLocaleString()}</p>
            </div>
            <p class="post-body">{@html postData.post.body}</p>
        </div>
    </div>
</section>

<!-- Create Comment -->
<section class="relative">
    {#if showCreateComment}
        <h2 class="text-2xl font-semibold mb-4">Add a Comment</h2>
        <form 
            method="post" 
            action="/api/comments" 
            bind:this={form}
            on:submit|preventDefault={handleSubmitComment}
        >
            <textarea bind:value={commentBody} disabled={loadingCommentSubmit} name="body" class="w-full px-4 py-2 mb-4 bg-secondary text-light border border-light rounded" rows="4" placeholder="Write your comment..." required></textarea>
            <button disabled={loadingCommentSubmit} type="submit" class="px-4 py-2 bg-accent text-light font-semibold rounded">
                {#if loadingCommentSubmit}
                    <div class="w-6 h-6 mx-auto border-t-2 border-light rounded-full animate-spin"></div>
                {:else}
                    Submit
                {/if}
            </button>
        </form>
    {:else}
        <h2 class="text-xl text-tertiary font-semibold">Login in to comment</h2>    
    {/if}
</section>
  
<!-- Comments section -->
<section class="mb-8">
    <h2 class="text-2xl font-semibold mb-4">Comments</h2>
    {#if comments.length === 0}
        <p class="text-lg text-accent font-semibold p-2">No comments yet... 😴</p>
    {/if}

    {#each comments as comment (comment.id)}
        <Comment data={comment} avatar_url={avatar_urls[comment.author_id]?.avatar_url} />
    {/each}
</section>


  