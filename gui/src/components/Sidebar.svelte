<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  import { quotes } from "../utils/constants";
  import type { Release } from "../global";

  let randomQuote;
  let versions: Release[] = [];
  let selectedVersion: Release = null;
  let buttonText;

  onMount(async () => {
    randomQuote = quotes[Math.floor(Math.random() * quotes.length)];
    versions = await invoke("get_versions");
  });

  $: loading = !versions;
  $: console.log(versions);

  $: {
    if (selectedVersion) {
      if (selectedVersion.installed) {
        buttonText = "Play";
      } else {
        buttonText = "Download";
      }
    } else buttonText = "Select Version";
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

  <img alt="Fish Fight logo" src="images/logo.png" class="logo mt-4" />

  {#if !loading}
    <div class="version-select">
      <label for="default_select">Versions</label>
      <div class="nes-select ">
        <select bind:value={selectedVersion} required id="default_select">
          <option value={null} selected>Select version</option>
          {#each versions as version}
            <option value={version}>{version.version}</option>
          {/each}
        </select>
      </div>
    </div>
    <button
      type="button"
      class="nes-btn is-warning play-btn"
      class:is-disabled={!selectedVersion}>{buttonText}</button
    >
  {/if}

  <nav class="social">
    <a target="_blank" href="https://twitter.com/fishfightgame"
      ><i class="nes-icon twitter" /></a
    >
    <a target="_blank" href="https://github.com/fishfight">
      <i class="nes-icon github " /></a
    >
    <a target="_blank" href="https://discord.gg/rKmE4HTD">
      <i class="nes-icon discord" /></a
    >
  </nav>
</section>

<style lang="scss">
  .sidebar {
    padding: 0px 20px;
    display: flex;
    flex-direction: column;
    flex: 3;
    background-color: white;
    height: 100vh;
    position: relative;

    .logo {
      align-self: center;
      width: 70%;
      margin-top: 10px;
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
