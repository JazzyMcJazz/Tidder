<script lang="ts">
	import { browser } from '$app/environment';
	import { goto } from '$app/navigation';
    import { page } from '$app/stores';

    $: nav = browser ? window : undefined;
    $: showAll = $page.url.searchParams.has('show_all');

    const handleShowAll = async () => {
        if (showAll) await goto($page.url.pathname.replace('?show_all=true', ''));
        else await goto($page.url.pathname + '?show_all=true');
        
        if (nav) nav?.location.reload();
    }

    

</script>

{#if $page.data.user?.role === 'admin'}
    <button 
        class="ml-4 text-sm text-accent hover:underline"
        on:click={handleShowAll}
    >
        {#if showAll}
            Hide Deleted/Drafts
        {:else}
            Show All
        {/if}
    </button>
{/if}