import { writable } from "svelte/store";
import type { StoreProgress } from "./global";

export const downloadProgress = writable<StoreProgress>({
  event: "idle",
  received: 0,
  total: 0,
  percent: 0,
});
