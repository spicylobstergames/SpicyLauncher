/// <reference types="svelte" />

import { GAMES, VERSIONING } from './utils/constants';

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
  body: string;
  version: string;
  installed: boolean;
  prerelease: boolean;
};

export type Game = keyof typeof GAMES;

export type Version = keyof typeof VERSIONING;
