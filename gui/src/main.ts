import App from "./App.svelte";
import { listen } from "@tauri-apps/api/event";
import { downloadProgress } from "./downloadStore";

type DownloadProgress = {
  received: number;
  total: number;
};

listen<DownloadProgress>("download", (event) => {
  console.log("download", event);

  const progress = (event.payload.received / event.payload.total) * 100;
  downloadProgress.set(progress);
});

import "nes.css/css/nes.min.css";

const app = new App({
  target: document.body,
  props: {
    name: "world",
  },
});

export default app;
