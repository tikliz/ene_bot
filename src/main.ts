import "./style.css";
import App from "./App.svelte";

import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';
import BmList from "./lib/BmList.svelte";

const app = new App({
  target: document.getElementById("app"),
});

// const result = await invoke('ADD_TO_LIST', {    
//   window: appWindow
// });

export default app;
