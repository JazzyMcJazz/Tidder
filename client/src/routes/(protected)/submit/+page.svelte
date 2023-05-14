<script lang="ts">
    import { page } from '$app/stores';
	import { goto } from '$app/navigation';
    import type { PageData } from './$types';
    import REST from '$lib/util/rest';
    
    export let data: PageData;

    let title = '';
    let body = '';

    let locked = $page.url.searchParams.has('subtidder');
    let checked = false;

    const { categories } = data;

    let form: HTMLFormElement;
    
    let loading = false;
    let error = '';
  
    const handleSubmit = async (draft?: boolean) => {
        loading = true;

        loading = true;
        error = '';

        const data = new FormData(form);
        const new_category = data.get('new_category') as string;
        const category_id = data.get('category_id') as string;
        const title = data.get('title') as string;
        const body = data.get('body') as string;

        const response = await REST.createPost({ title, body, category_id, new_category, draft });

        if (response.ok) {
            const data = await response.json();
            await goto(`/s/${data.category_id}/${data.post_id}`);
            
        } else {
            const data = await response.json();
            error = data.message
                .replace("Category", "Subtidder")
                .replace("category", "subtidder");

            setTimeout(() => {
                error = '';
            }, 5000);
        }
        
        loading = false;
    }
</script>

{#if error}
    <div class="error absolute top-32 right-20 p-2 w-fit font-semibold text-center rounded-lg opacity-90 bg-red-500 z-50">
        {error}
    </div>
{/if}

<section>
    <h1 class="text-4xl font-bold mb-8">Create a New Post</h1>
  
    <form 
        class="w-full"
        on:submit|preventDefault={() => handleSubmit()} 
        bind:this={form}
    >
        <div class="mb-4 w-72">
            <label for="subtidder" class="block text-lg font-semibold mb-2">Subtidder:</label>
            <div class="flex items-center">
                {#if checked}
                    <input disabled={loading} type="text" id="new_category" name="new_category" placeholder="New Subtidder" class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" required />
                {:else}
                    <select disabled={loading || locked} id="subtidder" name="category_id" class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" required>
                        {#if !locked}<option disabled selected>Select</option>{/if}
                        {#each categories as category}
                            <option value={category.id}>{category.name}</option>
                        {/each}
                    </select>
                    {#if locked}<input type="hidden" name="category_id" value={categories[0].id} />{/if}
                {/if}
                {#if !locked}
                    <input disabled={loading} type="checkbox" id="new" class="ml-4 mr-2" bind:checked={checked} />
                    <label for="new" class="text-light">New</label>
                {/if}
            </div>
        </div>

        <div class="mb-4">
            <label for="title" class="block text-lg font-semibold mb-2">Title:</label>
            <input disabled={loading} type="text" id="title" name="title" bind:value={title} class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" required />
        </div>
  
        <div class="mb-4 w-full">
            <label for="body" class="block text-lg font-semibold mb-2">Body:</label>
            <textarea disabled={loading} id="body" name="body" bind:value={body} class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" rows="10" required></textarea>
        </div>
  
        <div class="flex justify-between">
            <button
                disabled={loading}
                type="submit"
                class="px-4 py-2 bg-secondary text-light font-semibold rounded"
                on:click={() => handleSubmit(true)}
            >
                Save Draft
            </button>

            <button disabled={loading} type="submit" class="px-4 py-2 bg-accent text-light font-semibold rounded">
                {#if loading}
                    <div class="w-6 h-6 mx-4 border-t-2 border-light rounded-full animate-spin"></div>
                {:else}
                    Submit
                {/if}
            </button>
        </div>
    </form>
</section>