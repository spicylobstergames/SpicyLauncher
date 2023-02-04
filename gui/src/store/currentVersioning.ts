import { writable } from "svelte/store";
import type { Game } from "../global";
import { currentGame } from "./currentGame";

export const currentVersioning = writable<Game>("stable");

currentGame.subscribe((_) => {
  currentVersioning.update((_) => "stable");
});
