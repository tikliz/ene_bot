<script lang="ts" context="module">
	
    import { writable, type Writable } from 'svelte/store';
    import { AppRail, AppRailTile } from '@skeletonlabs/skeleton';
		

    export const storeValue: Writable<number> = writable(1);
    
</script>

<AppRail selected={storeValue} regionLead="w-fit h-fit" >
    <svelte:fragment slot="lead">
        <!-- <AppRailTile tag="a" href="/my/page/route">(icon)</AppRailTile> -->
    </svelte:fragment>
    <AppRailTile label="Tile" title="Tile" value={1}>(icon)</AppRailTile>
    <AppRailTile label="Tile" title="Tile" value={2}>(icon)</AppRailTile>
    <!-- <AppRailTile label="Tile" title="Tile" value={3}>(icon)</AppRailTile> -->
    <svelte:fragment slot="trail">
    <AppRailTile tag="a" href="/my/page/route">(icon)</AppRailTile>
    </svelte:fragment>
</AppRail>
    


<style>
	/* styles go here */
</style>