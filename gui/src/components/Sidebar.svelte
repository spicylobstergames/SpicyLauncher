<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api';

  import ProgressBar from './ProgressBar.svelte';

  import type { Release } from '../global';
  import versionStore from '../store/versionStore';

  import { currentGame } from '../store/currentGame';
  import { downloadProgress } from '../store/downloadStore';
  import { currentVersioning } from '../store/currentVersioning';
  import { GAMES, quotes, VERSIONING } from '../utils/constants';

  let buttonText: string;
  let gameTitle: string;
  let releaseType: string;
  let randomQuote: string;
  let selectedVersionNumber: string | null;

  $: loading = !$versionStore;
  $: gameTitle = GAMES[$currentGame];
  $: releaseType = VERSIONING[$currentVersioning];

  // Versioning
  $: selectedVersion = $versionStore.find(
    (v) => v.version === selectedVersionNumber
  );

  $: stableVersions = $versionStore.filter((v) => !v.prerelease);
  $: nightlyVersions = $versionStore.filter((v) => v.prerelease);
  $: versions =
    $currentVersioning === 'stable' ? stableVersions : nightlyVersions;

  // Version Checks
  $: hasVersions = versions.length > 0;

  $: btnDisabled =
    !selectedVersion ||
    ['Download', 'Extract'].includes($downloadProgress.event);

  $: {
    selectedVersionNumber = null;

    invoke<Release[]>('get_versions', { game: $currentGame }).then(
      (_versions: Release[]) => {
        versionStore.set(_versions);
        selectedVersionNumber = _versions.filter((v) =>
          $currentVersioning === 'stable' ? !v.prerelease : v.prerelease
        )[0]?.version;
      }
    );
  }

  $: {
    const index = $versionStore.findIndex(
      (v) => v.version === selectedVersionNumber
    );

    if (index !== -1) {
      const domVersions = document.querySelectorAll('.changelog h1');
      const target = domVersions[index];
      if (target) {
        target.scrollIntoView();
      }
    }
  }

  $: {
    if (selectedVersionNumber) {
      if (
        $versionStore.find((v) => v.version === selectedVersionNumber)
          ?.installed
      ) {
        buttonText = 'Play';
      } else {
        switch ($downloadProgress?.event) {
          case 'Download':
            buttonText = 'Downloading...';
            break;
          case 'Extract':
            buttonText = 'Extracting...';
            break;
          case 'Finished':
            buttonText = 'Play';
            invoke<Release[]>('get_versions', { game: $currentGame }).then(
              (v: Release[]) => {
                $versionStore = v;
              }
            );

            $downloadProgress.event = 'idle';
            break;
          default:
            buttonText = 'Download';
        }
      }
    } else buttonText = 'Select Version';
  }

  onMount(async () => {
    randomQuote = quotes[Math.floor(Math.random() * quotes.length)];
  });

  async function uninstallSelectedVersion () {
    if (selectedVersion?.installed) {
      await invoke('uninstall', {
        game: $currentGame,
        version: selectedVersion?.version
      });

      selectedVersion.installed = false;
      $downloadProgress.event = 'Finished';
    }
  }
</script>

<section class="sidebar">
  <div class="character-one">
    <div class="nes-balloon from-right message">
      {#if loading}
        <p>Loading...</p>
      {:else}
        <p>{randomQuote}</p>
      {/if}
    </div>
    <img src="/images/fish1.png" alt="character" />
  </div>

  <div>
    <h1 class="game-title">
      {gameTitle}
    </h1>

    {#if releaseType === VERSIONING.nightly}
      <p class="nes-badge pre-release">
        <span class="is-error">Pre-release</span>
      </p>
    {/if}
  </div>

  {#if !loading}
    <div class="nes-container with-title is-centered">
      <p class="title">Select Release Type</p>
      <label>
        <input
          type="radio"
          class="nes-radio"
          name="releases"
          value="stable"
          on:click={() => {
            selectedVersionNumber = null;
          }}
          bind:group={$currentVersioning}
        />
        <span>Stable</span>
      </label>

      <label>
        <input
          type="radio"
          class="nes-radio"
          name="releases"
          value="nightly"
          on:click={() => {
            selectedVersionNumber = null;
          }}
          bind:group={$currentVersioning}
        />
        <span>Pre-Release</span>
      </label>
    </div>

    <div class="version-select">
      <div class="nes-select">
        <select
          bind:value={selectedVersionNumber}
          required
          id="default_select"
          disabled={!hasVersions}
        >
          <option value={null} selected
            >{!hasVersions ? 'No Available Versions' : 'Select version'}</option
          >
          {#each versions as version}
            <option value={version.version}
              >{version.version} - {version.name}</option
            >
          {/each}
        </select>
      </div>
    </div>

    {#if !['Finished', 'idle'].includes($downloadProgress.event)}
      <ProgressBar progress={$downloadProgress.percent} />
    {/if}

    <button
      type="button"
      class="nes-btn is-warning play-btn"
      on:click={() => {
        invoke(selectedVersion?.installed ? 'launch' : 'install', {
          game: $currentGame,
          version: selectedVersion?.version
        });
      }}
      disabled={btnDisabled}
      class:is-disabled={btnDisabled}>{buttonText}</button
    >
    {#if selectedVersion && selectedVersion.installed}
      <button
        type="button"
        class="nes-btn is-warning play-btn"
        on:click={uninstallSelectedVersion}
        disabled={btnDisabled}
        class:is-disabled={btnDisabled}>Uninstall</button
      >
    {/if}
  {/if}

  <nav class="social">
    <a
      target="_blank"
      rel="noreferrer"
      href="https://twitter.com/spicylobsterfam"
      ><i class="nes-icon twitter" /></a
    >
    <a
      target="_blank"
      rel="noreferrer"
      href="https://github.com/spicylobstergames"
    >
      <i class="nes-icon github " /></a
    >
    <a target="_blank" rel="noreferrer" href="https://discord.gg/rKmE4HTD">
      <i class="nes-icon discord" /></a
    >
  </nav>
</section>

<style lang="scss">
  .pre-release {
    position: absolute;
    transform: rotate(25deg);
    right: 1rem;
    top: 1.5rem;
  }

  .sidebar {
    padding: 0px 20px;
    display: flex;
    flex-direction: column;
    flex: 3;
    background-color: white;
    position: relative;
    overflow: hidden;

    .game-title {
      font-size: 2.5em;
      text-align: center;
      margin-top: 0.75em;
    }

    .version-select {
      margin-top: 20px;
    }

    .play-btn {
      margin-top: 20px;
      background-color: #ffc061;
    }

    .character-one {
      position: fixed;

      width: 550px;
      bottom: -100px;
      right: -80px;
      display: flex;
      flex-direction: row;
      align-items: flex-start;
      justify-content: flex-end;

      .message {
        margin-top: -20px;
        margin-right: -10px;
      }
    }

    .social {
      position: absolute;
      bottom: 10px;
      left: 60px;
    }
  }
</style>
