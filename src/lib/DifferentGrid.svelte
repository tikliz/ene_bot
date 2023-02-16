<style>
:global(.svlt-grid-item) {
    background: rgb(129, 183, 187);

}

.size {
  max-width: 200px;
  width: 100%;
}
</style>

## Responsive - Serialize/Restore
<!-- <small><a target=_blank  href={`${openSource}/responsive-local-storage.svx`}>SOURCE</a></small> -->

This is a simple demo of responsiveness and localStorage.

> Please clear your local storage if you have problems with this example

----

<button on:click={reset}>Reset</button>

<div class="demo-container size">
  <Grid bind:items={items} rowHeight={100} let:item let:index {cols} on:mount={setCols} on:resize={setCols} on:pointerup={handleSync}>
    <div class=demo-widget>
     {index}
    </div>
  </Grid>
</div>

<script>

import Grid from "svelte-grid";
import gridHelp from "svelte-grid/build/helper/index.mjs";

const id = () => "_" + Math.random().toString(36).substr(2, 9);

let columns = 0;

const cols = [
  [400, 4],

];

let layoutOriginal = [
  {
    4: gridHelp.item({ x: 0, w: 1, h: 1, y: 0 }),
    //2: gridHelp.item({ x: 0, y: 0, w: 1, h: 1 }),
    id: id(),
  },

  {
    4: gridHelp.item({ x: 2, w: 1, h: 1, y: 0 }),
    //2: gridHelp.item({ x: 0, y: 2, w: 1, h: 1 }),
    id: id(),
  },
];

let layout = layoutOriginal;

if (typeof window !== "undefined") {
  if (!localStorage.getItem("layout-responsive")) {
    localStorage.setItem("layout-responsive", JSON.stringify(layoutOriginal));
  } else {
    layout = JSON.parse(localStorage.getItem("layout-responsive"));
  }
}

const handleSync = () => {
  localStorage.setItem("layout-responsive", JSON.stringify(items));
};

let items = layout;

const setCols = (e) => (columns = e.detail.cols);

const reset = () => {
  items = items.map((value, index) => {
    const restore = layoutOriginal[index][columns];
    return {
      ...value,
      [columns]: restore,
    };
  });

  localStorage.setItem("layout-responsive", JSON.stringify(items));
};
</script>