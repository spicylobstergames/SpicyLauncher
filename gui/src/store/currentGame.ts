import { writable } from "svelte/store";
import type { Game } from "../global";

export const currentGame = writable<Game>("jumpy");
