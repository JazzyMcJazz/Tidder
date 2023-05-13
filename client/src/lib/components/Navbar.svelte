<script lang="ts">
    import { page } from "$app/stores";
	import { PUBLIC_API_URL } from "$env/static/public";
	import { goto, invalidateAll } from "$app/navigation";
	import REST from "$lib/util/rest";
	import SignupModal from "./SignupModal.svelte";
	import Search from "./Search.svelte";

    $: user = $page.data.user;

    let showSignup = false;
    let loading = false;
    let error = false;

    let form: HTMLFormElement;

    const handleSubmit = async () => {
        loading = true;
        error = false;

        const data = new FormData(form);
        const username = data.get('username') as string;
        const password = data.get('password') as string;

        const response = await REST.login(username, password);
        
        if (response.ok) {
            await invalidateAll();
        } else {
            error = true;
        }
        
        loading = false;
    }

    const handleLogout = async () => {
        loading = true;
        await REST.logout();
        await invalidateAll();
        loading = false;
    }
    
</script>

{#if showSignup}
    <SignupModal onRequestClose={() => showSignup = false}/>
{/if}
  
<header class="px-4 py-4 mb-4 grid grid-cols-3 shadow-lg bg-primary">
    <!-- Logo -->
    <div class="flex items-center">
        <a href="/" class="text-2xl font-bold text-accent">tidder</a>
    </div>
  
    <!-- Search -->
    <Search />
  
    <!-- Auth -->
    {#if !!user}
        <div class="flex items-center justify-end">
            <a href="/account" class="text-lg font-semibold">My Account</a>
            <form 
                method="post" 
                action="{PUBLIC_API_URL}/api/logout"
                on:submit|preventDefault={handleLogout}
            >
                <button type="submit" class="w-20 px-4 py-2 bg-accent text-light font-semibold rounded-lg ml-2">
                    {#if loading}
                        <div class="w-6 h-6 mx-auto border-t-2 border-light rounded-full animate-spin"></div>
                    {:else}
                        Logout
                    {/if}
                </button>
            </form>
        </div>
    {:else}
        <form 
            method="post"
            action="{PUBLIC_API_URL}/api/login"
            class="flex items-center justify-end"
            enctype="application/x-www-form-urlencoded"
            bind:this={form}
            on:submit|preventDefault={handleSubmit}
        >
            <input disabled={loading} type="text" name="username" placeholder="Username" class="w-32 px-2 py-1 bg-secondary text-light border {error ? 'border-red-500' : 'border-light'} rounded-lg mr-1" />
            <input disabled={loading} type="password" name="password" placeholder="Password" class="w-32 px-2 py-1 bg-secondary text-light border {error ? 'border-red-500' : 'border-light'} rounded-lg mr-2" />
            <button disabled={loading} class="w-20 px-4 py-2 bg-accent text-light font-semibold rounded-lg">
                {#if loading}
                    <div class="w-6 h-6 mx-auto border-t-2 border-light rounded-full animate-spin"></div>
                {:else}
                    Login
                {/if}
            </button>
            <button type="button" class="ml-2 underline text-accent font-semibold" on:click={() => showSignup = true}>
                Register
            </button>
        </form>
    {/if}
</header>