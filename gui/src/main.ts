import App from "./App.svelte";

import { appWindow } from "@tauri-apps/api/window";
import { downloadProgress } from "./store/downloadStore";

appWindow.listen<Progress>("progress", (event) => {
  const progress = (event.payload.received / event.payload.total) * 100;
  downloadProgress.set({
    ...event.payload,
    percent: progress,
  });
});

import type { Progress } from "./global";

const app = new App({
  target: document.body,
});

export default app;
