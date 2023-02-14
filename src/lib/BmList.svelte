<script lang="ts">
    import { listen } from "@tauri-apps/api/event";

    
    // invoke vai ser usado dps, qnd precisarmos acessar a lista que vai estar no back-end
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import VirtualList from 'svelte-virtual-list';
    
    export let test = '';
    let list = [];
    let text_input = 'Silly input';
    let start;
    let end;


    const addToList = listen(
        'ADD_TO_LIST',
        ({event, payload}) => addItem(payload) //console.log(payload)
    );
    
    export function addItem(input) {
        list = [...list, input];

    }
    function removeItem() {
        list.shift();
        list = list;

    }

    // onMount(() => {
    //     listen('request', event => {
    //         list = [...list, event.payload];

    //     });

    // });

</script>


<style>
    button {
        border: none;
        padding: 0.5rem 2rem;
        color: #fff;
        font-size: 1.5rem;
        border-radius: 1rem;
        transition: all 250ms;
        transform-origin: center;
        box-shadow: 0px 3px 3px rgba(0, 0, 0, 0.25),
        inset 0px -2px 3px rgba(0, 0, 0, 0.25);
    }
    button:hover {
        cursor: pointer;
        transform: scale(0.975);
        box-shadow: 0px 1px 3px rgba(0, 0, 0, 0.25);
    }

    .container {
		border-top: 1px solid #333;
		border-bottom: 1px solid #333;
		min-height: 200px;
		height: calc(80vh - 15em);
	}
    .p_test {
        border-top: 1px solid #333;
        padding-top: 15px;

    }
    
    .button-container {
        padding-top: 5px;

    }

</style>

<h1>Listinha teste</h1>

<!-- <ul>
    { #each list as item}
        <li>{ item }</li>
    {/each}

</ul> -->
<div class="container">
    <VirtualList items = { list } bind:start bind:end let:item>
        <p>{ item }</p>

    </VirtualList>
    <p class="p_test">Showing items { start }-{ end }</p>

</div>

<p></p>
<input bind:value={ text_input }>
<div class="button-container">
    <!-- 'symbol' and 'bgColor' are props (green{"#39c41f"})  (red{"#c51f23"}) -->
    <button on:click={() => addItem("text_input") } style="background-color: #39c41f"> Add </button>
    <button on:click={ removeItem } style="background-color: #c51f23"> Remove </button>
</div>
<!-- <button on:click={ addItem }>ADD THIS TO LIST</button> -->