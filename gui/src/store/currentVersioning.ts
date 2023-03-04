import type { Version } from '../global'

import { writable } from 'svelte/store'

import { currentGame } from './currentGame'

export const currentVersioning = writable<Version>('stable')

currentGame.subscribe(_ => {
  currentVersioning.update(_ => 'stable')
})
