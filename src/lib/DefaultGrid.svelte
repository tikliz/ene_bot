<style>
    .demo-widget {
      background: #f1f1f1;
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
    </style>
    
    <label>
      <input type=checkbox bind:checked={fillFree} />
      'Fill space' is {fillFree ? 'enabled' : 'disabled'}
    </label>
    
    <div class=demo-container>
      <Grid bind:items={items} rowHeight={200} let:item let:dataItem {cols} fillSpace={fillFree}>
        <div class=demo-widget>{dataItem.id}</div>
      </Grid>
    </div>
    
    <script>
    import Grid from "svelte-grid";
    import gridHelp from "svelte-grid/build/helper/index.mjs";
    
    let fillFree = true;
    
    const id = () => "_" + Math.random().toString(36).substr(2, 9);
    
      const randomHexColorCode = () => {
        let n = (Math.random() * 0xfffff * 1000000).toString(16);
        return "#" + n.slice(0, 6);
      };
        
      function generateLayout(col) {
        return new Array(20).fill(null).map(function (item, i) {
          return {
            4: gridHelp.item({ x: (i * 2) % col, y: Math.floor(i / 6), w: 1, h: 1, resizable: false }),
            id: id(),
            data: randomHexColorCode(),
          };
        });
      }
        let what2 = 4;
        let what = what2*2;
        
      let cols = [
            [what, what2],
                             ];
    
      let items = gridHelp.adjust(generateLayout(4), what2);
    </script>