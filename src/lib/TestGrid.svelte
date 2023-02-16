<style>
  .demo-widget {
    background: #633838;
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  
  .demo-container {
    max-width: 1200px;;
    width: 100%;
  }
  .size {
    max-width: 1200px;
    width: 100%;
  }

  .remove { 
	  cursor: pointer;
	  position: absolute;
	  right: 5px; 
	  top: 3px;
	  user-select: none;
  }
  </style>

  
  <button on:click={reset}>Reset</button>
  <button on:click={localStore}>Save</button>
  <label>
    <input type=checkbox bind:checked={fillFree} />
    'Fill space' is {fillFree ? 'enabled' : 'disabled'}
  </label>
  
  <div class="demo-container size">
    <Grid bind:items={items} rowHeight={200} let:item let:dataItem {cols} fillSpace={fillFree} on:mount={setCols} on:resize={setCols} on:pointerup={handleSync}>
      <div class=demo-widget on:mousedown={() => findItem(dataItem) } on:mouseup={() => explodeItem(dataItem)}>{dataItem.data} - {dataItem.id} <br> {dataItem[4].x} {dataItem[4].y}</div>
    </Grid>
  </div>
  
  <script>
  import Grid from "svelte-grid";
  import gridHelp from "svelte-grid/build/helper/index.mjs";
  import {arrayMoveImmutable} from 'array-move';
    import { onMount } from "svelte";
  
  let fillFree = true;
  let propId;
  
  const id = () => "_" + Math.random().toString(36).substr(2, 9);
  
    const randomHexColorCode = () => {
      let n = (Math.random() * 0xfffff * 1000000).toString(16);
      return "#" + n.slice(0, 6);
    };
    let dataName = 0;
    function generateLayout(col) {
      return new Array(20).fill(null).map(function (item, i) {
        dataName += 1;
        return {
          4: gridHelp.item({ x: (i * 2) % col, y: Math.floor(i / 6), w: 1, h: 1, resizable: false }),
          id: id(),
          data: dataName,
        };
      });
    }
      let col2 = 4;
      let col1 = col2*2;
      let columns = 0;
      
    let cols = [
          [col1, col2],

    ];
  let layout = gridHelp.adjust(generateLayout(4), col2);
  let item_bkp = JSON.parse(localStorage.getItem("layout-responsive-2"));
  const localStore = () => {if (typeof window !== "undefined") {
    if (!localStorage.getItem("layout-responsive-2")) {
      localStorage.setItem("layout-responsive-2", JSON.stringify(layout));
    } else {
      layout = JSON.parse(localStorage.getItem("layout-responsive-2"));
      item_bkp = layout;
      //console.log("local store: ", layout);
    }
  }};
  localStore();
  
  
  const handleSync = () => {
    localStorage.setItem("layout-responsive-2", JSON.stringify(items));
    //console.log("blabla", JSON.parse(localStorage.getItem("layout-responsive-2")));
  };
  
  let items = layout;

  const setCols = (e) => (columns = e.detail.cols);

  const reset = () => {
  items = items.map((value, dataItem) => {
    const restore = layout[dataItem][columns];
    return {
      ...value,
      [columns]: restore,
    };
  });
  //console.log("reset: ", items);
};


let draggedItem;

function findItem(dataItem) {
  draggedItem = dataItem.id;
    //console.log("POG", dataItem.id);
}
function explodeItem(dataItem) {
  let test = items.findIndex((i) => i.data === dataItem.data);
  let pos = {x: items[test][4].x, y: items[test][4].y}
  let test2 = item_bkp.findIndex((i) => i[4].x == items[test][4].x && i[4].y == items[test][4].y);
  //console.log(i[4].x, dataItem[4].x);
  //let test3 = item_bkp.findIndex((i) => item_bkp[i].x === dataItem.x);
  if (Math.abs(test - test2) > 0) {
    let range = Math.abs(test - test2);
    console.log("range: ", range);
    for (let i = 0; i < range; i++) {
      //console.log("changed: ", item_bkp[test-i].data, " for ", item_bkp[test2-i].data);
      let temp_index = (test-i - test2-i);
      console.log(temp_index);
      items = arrayMoveImmutable(items, test-i, test2+i);

    }
    //localStore();


  }
    //console.log("vai tomar no cu");

  //item_bkp = JSON.parse(localStorage.getItem("layout-responsive-2"));
  // for (let i = 0; i < items.length; i++) {
  //   if (item_bkp[i][4] != items[i][4]) {
  //     items[i] = item_bkp[i];

  //   }
  //   //console.log(item_bkp[i][4].id);

  // }

  console.log("main index: ", test, "bkp index: ", test2, " data main: ", items[test], " data bkp: ", item_bkp[test]);
  // if (test === -3) {
  //   items.splice(test, test2);

  // }
  
  //console.log(items, item_bkp);

}
  //let items = gridHelp.adjust(generateLayout(4), col2);
  </script>