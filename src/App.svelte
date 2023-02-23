<script lang="ts">
  import '@skeletonlabs/skeleton/themes/theme-crimson.css';
  import '@skeletonlabs/skeleton/styles/all.css';
  import './app.postcss';	

  import SkeletonTest from './lib/SkeletonTest.svelte';
  import TestGrid from './lib/TestGrid.svelte';
  import {storeValue} from './lib/SkeletonTest.svelte';

  import { AppBar, AppRail, AppRailTile, AppShell, LightSwitch } from '@skeletonlabs/skeleton';
  import { writable, type Writable } from 'svelte/store';
  import { storePrefersDarkScheme, storeLightSwitch } from '@skeletonlabs/skeleton';

  let app_rail;
  
</script>

<main class="dark" data-theme="crimson">
  <AppShell slotSidebarLeft="bg-surface-500/5">
    <svelte:fragment slot="header">
      <AppBar>
      	<svelte:fragment slot="lead">(icon)</svelte:fragment>
      	(title)
      	<svelte:fragment slot="trail">(actions)</svelte:fragment>
      </AppBar>
				
    </svelte:fragment>
    <svelte:fragment slot="sidebarLeft">
    <SkeletonTest bind:this={app_rail}/>

    </svelte:fragment>
    <svelte:fragment slot="sidebarRight"></svelte:fragment>
    <svelte:fragment slot="pageHeader"></svelte:fragment>
    {#if $storeValue == 1}
      <TestGrid />

    {/if}

    <slot />
    <!-- <div class="container mx-auto p-8 space-y-8">
      <h1>Hello Skeleton</h1>
      <p>Lorem ipsum dolor sit amet consectetur adipisicing elit.</p>
      <hr />
      <section class="card p-4">
        <p>Lorem ipsum dolor sit amet consectetur adipisicing elit.</p>
      </section>
      <hr />
      <section class="flex space-x-2">
        <a class="btn variant-filled-primary" href="https://kit.svelte.dev/" target="_blank" rel="noreferrer">SvelteKit</a>
        <a class="btn variant-filled-secondary" href="https://tailwindcss.com/" target="_blank" rel="noreferrer">Tailwind</a>
        <a class="btn variant-filled-tertiary" href="https://github.com/" target="_blank" rel="noreferrer">GitHub</a>
      </section>
    </div> -->
    <svelte:fragment slot="pageFooter"></svelte:fragment>
    <svelte:fragment slot="footer"></svelte:fragment>
  </AppShell>
  <!-- <div>
    <TestGrid />
    <SkeletonTest />

  </div> -->
  

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>