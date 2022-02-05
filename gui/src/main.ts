import App from "./App.svelte";

import { appWindow } from "@tauri-apps/api/window";
import { downloadProgress } from "./downloadStore";

appWindow.listen<Progress>("progress", (event) => {
  const progress = (event.payload.received / event.payload.total) * 100;
  downloadProgress.set({
    ...event.payload,
    percent: progress,
  });
});

import "nes.css/css/nes.min.css";
import type { Progress } from "./global";

const app = new App({
  target: document.body,
  props: {
    name: "world",
  },
});

export default app;
