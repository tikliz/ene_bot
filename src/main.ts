import "./style.css";
import App from "./App.svelte";

import { invoke } from '@tauri-apps/api';
import { emit, listen } from '@tauri-apps/api/event';
import { appWindow } from '@tauri-apps/api/window';

const app = new App({
  target: document.getElementById("app"),
});

await listen(
  'ADD_TO_LIST',
  ({event, payload}) => console.log(payload)
);
const result = await invoke('do_some_long_task', {    
  window: appWindow
});

export default app;
