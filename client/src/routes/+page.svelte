<script lang="ts">
    import { slide } from 'svelte/transition';
	import { goto } from '$app/navigation';
    import { page } from '$app/stores';
	import PostPreview from '$lib/components/PostPreview.svelte';
	import ShowAllButton from '$lib/components/ShowAllButton.svelte';

    const { posts, categories } = $page.data;

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
    <section transition:slide|local>
        <h2 class="text-2xl font-semibold mb-4">Welcome {$page.data.user.username}</h2>
        <input 
            type="text" 
            placeholder="Create a new post" 
            class="w-full px-2 py-2 bg-secondary text-light border border-light rounded-lg mr-1" 
            on:click={() => goto('/submit')}
        />
    </section>
{/if}

<!-- Top Subtidders -->
<section>
    <h2 class="text-2xl font-semibold mb-4">Top Subtidders</h2>
    <ul class="grid grid-cols-2 md:grid-cols-4 gap-4">
        {#each categories as category}
            <a href="/s/{category.id}">
                <li class="p-4 bg-secondary rounded-lg">
                    <h3 class="text-lg font-semibold">{category.name}</h3>
                    <p class="text-sm">{category.posts} Post{category.posts === 1 ? '' : 's'}</p>
                </li>
            </a>
        {/each}
    </ul>
</section>

<!-- Hot Topics -->
<section>
    <div class="flex justify-between item-center">
        <h2 class="text-2xl font-semibold mb-4">Hot Topics</h2>
        <ShowAllButton />
    </div>
    {#each posts as post (post.id)}
        <PostPreview data={post}/>
    {/each}
</section>
  