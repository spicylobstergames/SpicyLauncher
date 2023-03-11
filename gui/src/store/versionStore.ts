import type { Release } from '../global'

import { writable, derived } from 'svelte/store'

const versionStore = writable<Release[]>([])

export const changelog = derived(versionStore, $versions => {
  return $versions
    .map(version => `# ${version.version} - ${version.name}\n${version.body}`)
    .join('\n\n')
})

export default versionStore
