import App from "./App.svelte";
import { listen } from "@tauri-apps/api/event";

listen("download", (event) => {
  console.log("download", event);
});

import "nes.css/css/nes.min.css";

const app = new App({
  target: document.body,
  props: {
    name: "world",
  },
});

export default app;
