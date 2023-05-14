<script lang="ts">
    import type { PageData } from './$types';
	import { goto } from '$app/navigation';
    import { page } from '$app/stores';
	import PostPreview from '$lib/components/PostPreview.svelte';
	import ShowAllButton from '$lib/components/ShowAllButton.svelte';
    
    export let data: PageData;

    const { category, posts } = data;

    let showSubmit = !!$page.data.user;
    $: switch (!!$page.data.user) {
        case true:
            setTimeout(() => {
                showSubmit = true;
            }, 500);
            break;
        default:
            showSubmit = false;
            break;
    }
</script>

<!-- Create Post -->
{#if showSubmit}
    <section>
        <input 
            type="text" 
            placeholder="Create a new post in {category.name}" 
            class="w-full px-2 py-2 bg-secondary text-light border border-light rounded-lg mr-1" 
            on:click={() => goto(`/submit?subtidder=${category.id}`)}
        />
    </section>
{/if}

<!-- Posts -->
<section class="mb-8">
    <div class="flex justify-between items-center">
        <div class="flex text-2xl font-semibold mb-4">
            <h2 class="mr-2">Welcome to</h2>
            <h2 class="text-accent">{category.name}</h2>
        </div>
        <ShowAllButton/>
    </div>
    {#each posts as post (post.id)}
        <PostPreview data={post}/>
    {/each}
</section>
