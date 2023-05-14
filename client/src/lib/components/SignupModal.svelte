<script lang="ts">
	import { PUBLIC_API_URL } from "$env/static/public";
	import { invalidateAll } from "$app/navigation";
    import REST from '$lib/util/rest';
    
    export let onRequestClose: () => void;

    let loading = false;
    let error = '';

    let form: HTMLFormElement;

    const handleSubmit = async () => {
        loading = true;

        loading = true;
        error = '';

        const data = new FormData(form);
        const username = data.get('username') as string;
        const password = data.get('password') as string;
        const confirm_password = data.get('confirm_password') as string;

        if (password !== confirm_password) {
            error = 'Passwords do not match.';
            loading = false;
            return;
        }

        const response = await REST.register(username, password);
        
        if (response.ok) {
            invalidateAll();
            onRequestClose();
        } else {
            const data = await response.json();
            error = data.message;
        }
        
        loading = false;
    }

</script>

<button 
    type="button" 
    class="absolute top-0 left-0 w-full h-full bg-primary bg-opacity-50 backdrop-blur-sm cursor-default z-40" 
    on:click={onRequestClose}
></button>

{#if error}
    <div class="error p-2 w-fit font-semibold text-center rounded-lg bg-red-500 z-50">
        {error}
    </div>
{/if}

<div class="modal flex flex-col justify-center items-center z-50">

    <div class="bg-secondary rounded-lg p-4">
        <h2 class="text-2xl font-semibold mb-4">Sign Up</h2>
        <form 
            class="w-full" 
            method="post" 
            action="{PUBLIC_API_URL}/api/register" 
            on:submit|preventDefault={handleSubmit}
            bind:this={form}
        >
            <div class="mb-4">
                <label for="username" class="block text-lg font-semibold mb-2">Username:</label>
                <input disabled={loading} type="text" id="username" name="username" class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" required />
            </div>

            <div class="mb-4">
                <label for="password" class="block text-lg font-semibold mb-2">Password:</label>
                <input disabled={loading} type="password" id="password" name="password" class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" required />
            </div>

            <div class="mb-4">
                <label for="confirm_password" class="block text-lg font-semibold mb-2">Confirm Password:</label>
                <input disabled={loading} type="password" id="confirm_password" name="confirm_password" class="w-full px-4 py-2 bg-secondary text-light border border-light rounded" required />
            </div>

            <div class="flex justify-between items-center">
                    <button disabled={loading} type="button" class="px-4 py-2 bg-tertiary text-light font-semibold rounded" on:click={onRequestClose}>Cancel</button>
                    <button  type="submit" class="px-4 py-2 bg-accent text-light font-semibold rounded">
                        {#if loading}
                            <div class="w-6 h-6 mx-4 border-t-2 border-light rounded-full animate-spin"></div>
                        {:else}
                            Submit
                        {/if}
                    </button>
            </div>

        </form>
    </div>
</div>

<style>
    .modal {
        position: fixed;
        top: calc(50% - 10rem);
        left: calc(50% - 10rem);
    }

    .error {
        position: fixed;
        top: 4rem;
        width: 19rem;
        left: calc(50% - 10rem);
    }
</style>