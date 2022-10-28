<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  import type { Release } from "../global";
  import ProgressBar from "./ProgressBar.svelte";
  import { downloadProgress } from "../downloadStore";
  import versionStore from "../versionStore";
  import { quotes } from "../utils/constants";

  let randomQuote;
  let selectedVersionNumber;
  let buttonText;

  onMount(async () => {
    randomQuote = quotes[Math.floor(Math.random() * quotes.length)];
    versionStore.set(await invoke("get_versions"));
  });

  $: loading = !$versionStore;

  $: selectedVersion = $versionStore.find(
    (v) => v.version === selectedVersionNumber
  );

  $: {
    const index = $versionStore.findIndex(
      (v) => v.version === selectedVersionNumber
    );
    if (index !== -1) {
      const domVersions = document.querySelectorAll(".changelog h1");
      const target = domVersions[index];
      target.scrollIntoView();
    }
  }

  $: {
    if (selectedVersionNumber) {
      if (
        $versionStore.find((v) => v.version === selectedVersionNumber).installed
      ) {
        buttonText = "Play";
      } else {
        if ($downloadProgress?.event === "Download") {
          buttonText = "Downloading...";
        } else if ($downloadProgress?.event === "Extract") {
          buttonText = "Extracting...";
        } else if ($downloadProgress?.event === "Finished") {
          buttonText = "Play";
          invoke("get_versions").then((v: Release[]) => {
            console.log("v", v);
            $versionStore = v;
          });

          $downloadProgress.event = "idle";
        } else {
          buttonText = "Download";
        }
      }
    } else buttonText = "Select Version";
  }
  $: btnDisabled =
    !selectedVersion ||
    ["Download", "Extract"].includes($downloadProgress.event);
  $: console.log($downloadProgress);
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
        <select bind:value={selectedVersionNumber} required id="default_select">
          <option value={null} selected>Select version</option>
          {#each $versionStore as version}
            <option value={version.version}
              >{version.version} - {version.name}</option
            >
          {/each}
        </select>
      </div>
    </div>

    {#if !["Finished", "idle"].includes($downloadProgress.event)}
      <ProgressBar progress={$downloadProgress.percent} />
    {/if}

    <button
      type="button"
      class="nes-btn is-warning play-btn"
      on:click={() => {
        invoke(selectedVersion.installed ? "launch" : "install", {
          version: selectedVersion.version,
        });
      }}
      disabled={btnDisabled}
      class:is-disabled={btnDisabled}>{buttonText}</button
    >
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
