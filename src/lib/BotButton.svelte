<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api/tauri";
    import { emit } from "@tauri-apps/api/event";

    let yes = true;
    let test = "";
    async function irc_client(){
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        test = await invoke("irc_client2")
    }
    onMount(irc_client);

    function handleCheckboxChange(event) {
      emit("handle_bot_checkbox", event.target.checked);

    }

  </script>
  
  <div>
    <div class="row">
      <input type=checkbox on:change={handleCheckboxChange} bind:checked={yes}>
        {#if yes}
            <p>BOT RESPONSE ENABLED</p>
        {:else}
            <p>BOT RESPONSE DISABLED</p>
        {/if}
    </div>
  </div>