<script lang="ts">
	import { onMount } from "svelte";

    let visible = false;
    let menu: HTMLDivElement;
    let button: HTMLDivElement;
  
    function toggle() {
      visible = !visible;
    }

    function clickOutside (event: any) {
        if (menu && button && !menu.contains(event.target) && !button.contains(event.target)) {
            visible = false;
        }
    }

    onMount(() => {
        document.addEventListener('click', clickOutside);
    });
</script>
  
<div class="relative" bind:this={button}>
    <div>
        <button class="flex items-center justify-center w-8 h-8 hover:bg-primary hover:brightness-125 rounded"
        on:click={toggle}>
        <span class="text-2xl text-tertiary">•••</span>
        </button>
    </div>

    <div 
        bind:this={menu} 
        class="absolute right-0 mt-2 rounded-md shadow-lg bg-secondary z-50" 
        hidden={!visible} 
    >
        <slot/>   
    </div>
</div>
  
  