<style>
  :global(.svlt-grid-shadow) {
    /* Back shadow */
    background: rgb(212, 22, 60) !important;
  }

  button {
    padding: 1%;
    color: #fff;
    background-color: rgb(212, 22, 60);
    margin-bottom: 5px;
    margin-left: 5px;
    margin-top: 5px;

  }

  .demo-widget {
    /* background: var(--bg, #f4f4f4); */
    background-color: rgb(49, 57, 65);
    height: 100%;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px solid black;
    border-left: 2px solid black;
    border-right: 2px solid black;
    /* box-shadow: 12px 12px rgba(212, 22, 60, 0.4); */
  }
  
  .demo-container {
    padding: auto;
    min-width: 1300px;
    max-width: 1920px;
    min-height: 620px;
    height: max-content;
    width: 100%;
    border: 1px solid black;
    overflow: scroll;
  }
  .size {
    max-width: 1300px;
    min-width: 1300px;
    width: 100%;
  }

  .demo-widget p {
    user-select: none;

  }
  .bd {
    color: white;
    width: 100%; height: 100%;
    /* background-color: rgb(49, 57, 65); */
    user-select: none;
    position: relative;
    justify-content: center;
    background-color: rgba(49, 57, 65, 0.3);
    background-image: var(--mapBg);
    background-size: 200%;
    background-repeat: no-repeat;
    background-position: center;
    border-left: 3px solid var(--bg);
    background-blend-mode: darken;

  }
  .bd p {
    position: absolute;
    color: rgb(255, 255, 255);
    opacity: 1;
    text-shadow: 1px 1px 1px black;
    font-family: Arial, Helvetica, sans-serif;
    pointer-events: none;
    
  }
  .title {
    text-align: left;
    position: absolute;
    /* font-size: 120%; */
    line-height: 1.2;
    /* font-size-adjust: 50%; */
    padding-bottom: 20px;
    top: 20px;
    left: 10px;
    right: 2px;
    pointer-events: none;
    font-weight: bold;

  }
  .mapper {
    word-wrap: break-word;
    text-align: right;
    font-size: 70%;
    position: absolute;
    right: 10px;
    /* left: 65px; */
    bottom: 25px;
    pointer-events: none;

  }
  .requester {
    text-align: right;
    font-size: 80%;
    color: var(--bg, black) !important;
    position: absolute;
    bottom: -3px;
    right: 9px;
    text-shadow: none !important;
    pointer-events: none;

  }
  .bpm {
    text-align: left;
    position: absolute;
    font-size: 180%;
    left: 30px;
    word-spacing: -6px;
    left: -3px;
    bottom: 30px;
    padding-top: 4px;
    padding-bottom: 4px;
    padding-right: 6px;
    pointer-events: none;
    padding-left: 8px;
    background-color: var(--bg, black) !important;

  }

  .right-side  {
    width: 15%;
    height: 100%;
    background: var(--bg, #f4f4f4);
    /* line-height: 15px; */

  }

  .count {
    display: flex;
    /* position: absolute; bottom: 0; right: 0; */
    align-items: center;
    /* height: 75%; */
    right: 14px;
    justify-content: center;
    /* left: 90%; */
    background-color: red;

  }
  .count p {
    /* padding-top: 125%; */
    color: black;
    border-bottom: 1px solid black;
    text-align: center;
    position: absolute;
    bottom: 15px;

  }

  .remove { 
    background-color: var(--bg, #f4f4f4);;
    color: black;
    /* background-color: aliceblue; */
	  cursor: pointer;
	  border-top: 1px solid black;
    position: absolute;
    top: 0;
    right: 2px;

    height: 15%;
    width: 12%;
	  /* right: 2%;  */
	  /* top: 6px; */
    /* bottom: 200px; */
	  user-select: none;
    /* padding-bottom: 90%; */
  }
  .remove p {
    position: absolute;
    align-items: center;
    top: 0px;
    right: 13.5px;

  }

  </style>

  
  <button on:click={loadLocalStore}>Reset</button>
  <button on:click={saveLocalStore}>Save</button>
  <!-- <button on:click={adjustAndSave}>Save safety backup</button> -->
  <button on:click={undoOne}>Undo</button>
  
  <div class="demo-container size">
    <Grid bind:items={items} rowHeight={200} let:item let:dataItem {cols} fillSpace={true} on:mount={setCols} on:resize={setCols} on:pointerup={handleSync} let:movePointerDown>
      <div class=demo-widget style="--bg: {dataItem.bg}; --mapBg: url({dataItem.mapBg})" on:mouseup={() => moveItem(dataItem)}>
                                                                                                            <!-- on:mouseup={() => adjustAndSave()} -->
      <a class="bd" href="{dataItem.url}" target="_blank" rel="noreferrer">
        <div class="bd">
          <p class="title" use:textfit={{mode:"single", width: 1000, height:100,
          forceSingleModeWidth:false}}>{dataItem.mapTitle} <b>\\</b> {dataItem.mapArtist}</p>
          <p class="mapper">mapeado por {dataItem.mapper}</p>
          <p class="requester">{dataItem.requester}</p>
          <p class="bpm">{dataItem.requester_badge} {dataItem.mapBpm}</p>
        
        </div>

      </a>
        <!-- remove self button -->
        <div class="right-side" on:pointerdown={movePointerDown}><span on:pointerdown={e => e.stopPropagation()} 
          on:mousedown={() => remove(dataItem)}
          class="remove"
          >
          <p>X</p>
        </span>
        <div class="count"><p>{items.indexOf(dataItem)}</p></div>
      </div>

      </div>
    </Grid>
  </div>
  
  <script lang="ts">
  import Grid from "svelte-grid";
  import gridHelp from "svelte-grid/build/helper/index.mjs";
  import {arrayMoveImmutable} from 'array-move';
  import { listen } from "@tauri-apps/api/event";
  import { ArrayWHistory } from "../undoChanges";
  import { textfit } from 'svelte-textfit';
  import { onMount } from "svelte";

  // events 
  const addToList = listen(
        'ADD_TO_LIST',
        ({event, payload}) =>  addItem(payload), //console.log(payload),
    );
  
  // other stuff
  
  let propId;
  const COLS = 4;
  
  const id = () => "_" + Math.random().toString(36).substr(2, 9);
  
  const randomHexColorCode = () => {
    const r = Math.floor(Math.random() * 256);
    const g = Math.floor(Math.random() * 256);
    const b = Math.floor(Math.random() * 256);
    return `rgb(${r}, ${g}, ${b})`;
  };

  function addItem(payload) {
    // console.log(payload);
    let item = {
        [COLS]: gridHelp.item({ w: 1, h: 1, resizable: false, customDragger: true, }),
        // item id, not beatmap id
        id: id(),
        bg: randomHexColorCode(),
        url: payload.url,
        mapBg: payload.map_bg,
        mapTitle: payload.map_title,
        mapArtist: payload.map_artist,
        mapBpm: payload.map_bpm,
        mapper: payload.mapper,
        requester: payload.requester,
        requester_badge: payload.badge,

      };

    let findOutPosition = gridHelp.findSpace(item, items, COLS);
    item = {
      ...item,
      [COLS]: {
        ...item[COLS],
        ...findOutPosition,

      },
      
    };
    items = [...items, ...[item]];
    items = gridHelp.normalize(items, COLS);
    let original_order = parseLocalStore();
    let all_reqs = parseLocalReqs();
    all_reqs = [...all_reqs, ...[item]];
    original_order = [...original_order, ...[item]];
    layout.push(...[item]);

    adjustAndSave();
    
    //console.log(localStorage.getItem("original-order"));
    localStorage.setItem("original-order", JSON.stringify(original_order));
    localStorage.setItem("local-reqs", JSON.stringify(all_reqs));
    //console.log(localStorage.getItem("original-order"));

    }
  
  // generate default/saved layout when we reset things

  function generateLayout(col) {
    if (localStorage.getItem("safety-backup")) {
    let json = JSON.parse(localStorage.getItem("safety-backup"));
    return json.map((value, dataItem) => {
    const restore = json[dataItem][columns];
    return {
      ...value,
      [columns]: restore,
    };
  });

    } else {
      return new Array(1).fill(null).map(function (item, i) {
      return {
        [COLS]: gridHelp.item({ x: (i * 2) % col, y: Math.floor(i / 6), w: 1, h: 1, resizable: false, customDragger: true, }),
        id: id(),
        bg: randomHexColorCode(),
        url: "https://osu.ppy.sh/beatmapsets/351630#osu/774965",
        mapBg: "https://assets.ppy.sh/beatmaps/351630/covers/cover.jpg?1622092984",
        mapTitle: "Remote Control",
        mapArtist: "kradness&Reol",
        mapBpm: "165",
        mapper: "Taeyang",
        requester: "rheniitassadadfgewfwe",
        requester_badge: "★"

      };

    });

    }

  }
    let col1 = COLS*2;
    let columns = 0;
      
    let cols = [
      [col1, COLS],

    ];
    
  function parseLocalReqs() {
    let json = JSON.parse(localStorage.getItem("local-reqs"));
      return json.map((value, dataItem) => {
        const restore = json[dataItem][columns];
        return {
          ...value,
          [columns]: restore,

        };

      });

  }
  
  function parseLocalStore() {
    let json = JSON.parse(localStorage.getItem("original-order"));
      return json.map((value, dataItem) => {
        const restore = json[dataItem][columns];
        return {
          ...value,
          [columns]: restore,

        };

      });

  }
  const layout = new ArrayWHistory(gridHelp.adjust(generateLayout(COLS), COLS), "undo-history");
  
  function backupLocalStore() {
    

  }
  const saveLocalStore = () => {
    if (localStorage.getItem("original-order")) {
      localStorage.setItem("original-order", JSON.stringify(layout.toArray()));

    }};
  const loadLocalStore = () => {if (typeof window !== "undefined") {
    if (!localStorage.getItem("original-order")) {
      localStorage.setItem("local-reqs", JSON.stringify(items));
      localStorage.setItem("original-order", JSON.stringify(items));
    } else {
      //layout = JSON.parse(localStorage.getItem("layout-responsive-2"));
      let json = parseLocalReqs();
      items = json;
      //items = gridHelp.normalize(items, COLS);
      items = gridHelp.adjust(items, COLS);
      layout.replace(items);
      undoOne(); undoOne();
      //item_bkp = layout;
      //console.log("local store: ", layout);
    }
  }};
  const handleSync = () => {
    console.log(items);
    localStorage.setItem("safety-backup", JSON.stringify(items));

  };
  
  let items = layout.toArray();

  const setCols = (e) => (columns = e.detail.cols);

  const adjustList = () => {
    items = gridHelp.adjust(items, COLS);

  }
  const adjustAndSave = () => {
    if (!localStorage.getItem("safety-backup")) {
      console.log("booting for first time. maybe");
      adjustList();
      localStorage.setItem("original-order", JSON.stringify(items));
      localStorage.setItem("local-reqs", JSON.stringify(items));
      localStorage.setItem("safety-backup", JSON.stringify(items));
      
    } else {
      adjustList();
      localStorage.setItem("safety-backup", JSON.stringify(items));

    }

  }
  
  const undoOne = () => {
    let original_order = parseLocalStore();
    let all_reqs = parseLocalReqs();
    layout.undo();
    items = layout.toArray();
    gridHelp.normalize(items, COLS);
    adjustList();
    if (original_order < all_reqs) {
      localStorage.setItem("original-order", JSON.stringify(all_reqs));

    }
    

  }
  const loadSafety = () => {if (typeof window !== "undefined") {
   if (localStorage.getItem("safety-backup")) {
     console.log("loading ACTUAL backup");
     let layout_fail_backup = JSON.parse(localStorage.getItem("safety-backup"));
     items = layout_fail_backup.map((value, dataItem) => {
       const restore = layout.toArray()[dataItem][columns];
       return {
         ...value,
         [columns]: restore,

       };
      

   });
   adjustList();

  } else {
    console.log("didn't find a backup.");
    adjustAndSave();

  }

}};

function moveItem(dataItem) {
  let main_index = items.findIndex((i) => i == dataItem);
  let bkp_index = (JSON.parse(localStorage.getItem("original-order"))).findIndex((i) => i[COLS].x == items[main_index][COLS].x && i[COLS].y == items[main_index][COLS].y);
  let all_reqs = parseLocalReqs();

  //let pos = {x: items[main_index][4].x, y: items[main_index][4].y};
  //console.log(i[4].x, dataItem[4].x);
  if (Math.abs(main_index - bkp_index) > 0) {
  if (items.length > all_reqs.length) {
    for (let i = 0; i < items.length; i++) {
      if (all_reqs.findIndex((order) => order === items[i]) === -1) {
        console.log("updating all reqs with: ", items[i]);
        all_reqs = [...all_reqs, ...[items[i]]];
        localStorage.setItem("local-reqs", JSON.stringify(all_reqs));

      }}
  } else if (items.length !== all_reqs.length) {
    let temp;;
    for (let i = 0; i < all_reqs.length; i++) {      
      if (all_reqs.findIndex((order) => order === items[i]) === -1) {
        temp = i;
        break;

      }
      

  }
    all_reqs.splice(temp, 1);
    localStorage.setItem("local-reqs", JSON.stringify(all_reqs));

  }
    
    let range = Math.abs(main_index - bkp_index);
    //console.log("range: ", range);
    for (let i = 0; i < range; i++) {
      let index_1 = bkp_index;
      let index_2 = main_index;
      //console.log(index_1, index_2);
      items = arrayMoveImmutable(items, index_1, index_2);
      

    }
    layout.replace(items);
    adjustAndSave();

  }
  //console.log("main index: ", main_index, "bkp index: ", bkp_index, " data main: ", items[main_index], " data bkp: ", item_bkp[bkp_index]);

}
  const remove = (item) => {
    let original_order = parseLocalStore();
    let index = original_order.findIndex((i) => i == item);
    layout.splice(index);
    original_order = original_order.filter((value) => value.id !== item.id);
    original_order = gridHelp.adjust(original_order, COLS);
    localStorage.setItem("original-order", JSON.stringify(original_order));

    items = items.filter((value) => value !== item);
    adjustList();
    console.log(items, layout.toArray());

  };
  //let items = gridHelp.adjust(generateLayout(4), COLS);
  loadSafety();
  </script>