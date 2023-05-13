<script lang="ts">
    import REST from '$lib/util/rest';
	import type { Category, Post } from '$lib/util/types';

    let form: HTMLFormElement;
    let input: HTMLInputElement;
    let timeout: NodeJS.Timeout;

    let showResults = false;
    let loading = false;
    let results = {
        categories: [] as Category[],
        posts: [] as Post[],
    };

    const handleSearch = async () => {
        clearTimeout(timeout);

        showResults = true;
        loading = true;

        const data = new FormData(form);
        const query = data.get('query') as string;
        results = await REST.search(query);
        loading = false;
    }

    const handleSearchChange = () => {
        clearTimeout(timeout);
        const data = new FormData(form);
        const query = data.get('query') as string;
        if (query.length === 0) {
            showResults = false;
            loading = false;
            return;
        }

        timeout = setTimeout(() => {
            handleSearch();
        }, 500);
    }

    const onFocus = () => {
        if (form.query.value.length > 0) {
            showResults = true;
        }
    }
</script>

{#if showResults}
    <button 
        type="button" 
        class="absolute top-0 left-0 w-full h-full cursor-default z-40"
        on:click={() => showResults = false}
    ></button>
{/if}

<div class="relative flex flex-col items-center justify-center w-full">
    <form
        action="/api/search" 
        on:submit|preventDefault={handleSearch} 
        bind:this={form}
        class="w-full max-w-xs z-50"
    >
        <input 
            type="text" 
            name="query" 
            placeholder="Search..." 
            autocomplete="off"
            on:input={handleSearchChange}
            on:focus={onFocus}
            class="w-full px-4 py-2 bg-secondary text-light border border-light {showResults ? 'rounded-t-lg' : 'rounded-lg'}" 
        >
    </form>

    {#if showResults}
        <!-- search results -->
        <div class="absolute top-11 w-full h-72 max-w-xs bg-secondary rounded-b-lg border border-light overflow-y-auto z-50">
            {#if loading}
                <div class="flex items-center justify-center h-full">
                    <div class="w-6 h-6 mx-auto border-t-2 border-light rounded-full animate-spin"></div>
                </div>
            {:else if results.categories.length === 0 && results.posts.length === 0}
                <div class="flex flex-col h-full justify-center items-center">
                    <p class="text-center text-accent font-semibold p-2">We couldn't find the results you were looking for. We're sorry..</p>
                    <p class="text-2xl">🥲</p>
                </div>
            {:else}
                {#if results.categories.length > 0}<p class="text-lg text-accent font-bold p-2">Subtidders</p>{/if}
                {#each results.categories as category}
                    <a 
                        href="/s/{category.id}"
                        class="block w-full font-bold p-2 hover:bg-tertiary"
                        on:click={() => showResults = false}
                    >
                        {category.name}
                    </a>
                {/each}

                {#if results.categories.length > 0 && results.posts.length > 0}
                    <hr class="border-light" />
                {/if}

                {#if results.posts.length > 0}<p class="text-lg text-accent font-bold p-2">Posts</p>{/if}
                {#each results.posts as post}
                    <a 
                        href="/s/{post.category_id}/{post.id}"
                        class="block w-full font-bold p-2 hover:bg-tertiary"
                        on:click={() => showResults = false}
                    >
                        <div class="">
                            <div class="flex items-center">
                                <p class="post-author font-semibold">{post.author_name}</p>
                                <p class="post-date ml-2 text-xs">• in {post.category_name}</p>
                            </div>
                            <p>{post.title}</p>
                        </div>
                    </a>
                {/each}
            {/if}
        </div>
        
    {/if}
</div>