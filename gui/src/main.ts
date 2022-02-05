import App from "./App.svelte";

import { appWindow } from "@tauri-apps/api/window";
import { downloadProgress } from "./downloadStore";

type DownloadProgress = {
  received: number;
  total: number;
};

appWindow.listen<DownloadProgress>("progress", (event) => {
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
