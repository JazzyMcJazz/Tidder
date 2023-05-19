<script lang="ts">
    import type { Post } from '$lib/util/types';
    import { page } from '$app/stores';
	import KebabMenu from '$lib/components/KebabMenu.svelte';
    import ChevronDown from 'svelte-icons/fa/FaChevronDown.svelte'
    import REST from '$lib/util/rest';
	import { invalidateAll } from '$app/navigation';
	import { PUBLIC_API_URL } from '$env/static/public';

    $: posts = $page.data.posts as Post[];
    $: user = $page.data.user;
    $: avatar_url = $page.data.avatar_url?.startsWith(PUBLIC_API_URL) 
                    ? $page.data.avatar_url 
                    : undefined

    let avatarInput: HTMLInputElement;

    let loading = '';
    let loadAvatar = false;

    let showPosts = false;

    const handleUploadAvatar = async () => {
        loadAvatar = true;
        if (avatarInput.files?.length !== 1) {
            loadAvatar = false;
            return;
        }

        const file = avatarInput.files[0];
        await REST.uploadAvatar(file);
        await invalidateAll();
        
        loadAvatar = false;
    }

    const handlePublish = async (postId: string) => {
        loading = postId;
        await REST.publishPost(postId);
        await invalidateAll();
        loading = '';
    }

    const handleDelete = async (postId: string) => {
        loading = postId;
        await REST.deletePost(postId);
        await invalidateAll();
        loading = '';
    }
</script>

<section class="flex">
    <h1 class="text-accent text-4xl font-bold">{user?.username}</h1>
    <h1 class="text-4xl font-bold">'s Account</h1>
</section>

<section>
    <h1 class="text-2xl font-semibold mb-4">Avatar</h1>
   
    <form 
        class="flex items-center mt-4" 
        method="post"
        action="{PUBLIC_API_URL}/api/avatar/upload"
        enctype="multipart/form-data"
        on:submit|preventDefault={handleUploadAvatar}
        
    >
        <label class="relative group flex items-center justify-center border-2 border-accent first-letter: rounded-full text-light hover:cursor-pointer">
            <input 
                type="file" 
                class="hidden" 
                multiple={false}
                accept="image/png, image/jpeg, image/gif"
                bind:this={avatarInput}
                on:change={handleUploadAvatar}
            />
            <img 
                src={avatar_url || '/fallback.png'} 
                alt="avatar" 
                class="w-24 h-24 object-cover rounded-full group-hover:brightness-50 {loadAvatar ? 'brightness-50' : ''}"
            />

            {#if loadAvatar}
                <div class="absolute flex justify-center items-center">
                    <div class="w-10 h-10 mx-auto border-t-2 border-accent rounded-full animate-spin"></div>
                </div>
            {:else}
                <div class="absolute hidden group-hover:flex jusify-center text-center text-light font-semibold">
                    Upload
                </div>
            {/if}
        </label>
    </form>

</section>
    
<section>
    
        
        <button 
            on:click={() => showPosts = !showPosts}
            class="w-full hover:cursor-pointer flex justify-between items-center " 
        >
            <h2 class="text-2xl font-semibold mb-4">Your Posts</h2>
            <div class="w-6 transform rotate-{showPosts ? '180' : '0'} transition-transform duration-300">
                <ChevronDown />
            </div>
        </button>
            
    

    {#if showPosts}
        <div>

            {#if !posts.length}
                <p class="text-light">You haven't posted anything yet.</p>
            {/if}
            {#each posts as post}
                <div class="relative flex items-center">
                    <a 
                        href="s/{post.category_id}/{post.id}"
                        class="post group flex flex-1 justify-between items-center hover:brightness-125 bg-primary p-4"
                    >
                        <div>
                            <div class="flex">
                                {#if !post.published}
                                    <h3 class="post-title text-accent mr-1">[draft]</h3>
                                {/if}
                                <h3 class="post-title">{@html post.title}</h3>
                            </div>
                            <p class="post-date">{new Date(post.created_at).toLocaleString()}</p>
                        </div>
                            
                    </a>
                    <div class="absolute right-2">
                        {#if loading === post.id}
                            <div class="mr-1">
                                <div class="w-6 h-6 mx-auto border-t-2 border-accent rounded-full animate-spin"></div>
                            </div>
                        {:else if !post.deleted}
                            <KebabMenu>
                                <div class="flex flex-col items-start justify-start p-4 w-48">
                                    {#if !post.published}
                                        <button 
                                            class="text-sm text-accent hover:underline"
                                            on:click={() => handlePublish(post.id)}
                                        >
                                            Publish draft
                                        </button>
                                    {/if}
                                    <button 
                                        class="text-sm text-red-500 hover:underline"
                                        on:click={() => handleDelete(post.id)}
                                    >
                                        Delete
                                    </button>
                                </div>
                            </KebabMenu>
                        {/if}
                    </div>
                </div>
            {/each}
        </div>

    {/if}
</section>
