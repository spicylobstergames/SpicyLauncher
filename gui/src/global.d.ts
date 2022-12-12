/// <reference types="svelte" />

type Progress = {
  event: "Download" | "Extract" | "Finished" | "idle";
  received: number;
  total: number;
};

type StoreProgress = Progress & {
  percent: number;
};

export type Release = {
  name: string;
  version: string;
  body: string;
  installed: boolean;
};

export type Game = keyof typeof GAMES
