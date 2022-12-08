<script lang="ts">
  import SvelteMarkdown from "svelte-markdown";
  import { changelog } from "../versionStore";
  import LinkRenderer from "./LinkRenderer.svelte";
</script>

<div class="changelog">
  <img src="/images/fish2.png" alt="character" class="character-two" />
  <img src="/images/fish3.png" alt="character" class="character-three" />

  <div class="changelog__wrapper">
    <SvelteMarkdown source={$changelog} renderers={{ link: LinkRenderer }} />
  </div>
</div>

<style lang="scss">
  .changelog {
    flex: 5;
    position: relative;
    width: 20%;

    &__wrapper {
      padding: 15px;
      height: 100%;
      word-wrap: break-word;
      background-color: rgba(0, 0, 0, 0.5);
      color: white;
      overflow-y: auto;

      :global(h1) {
        color: #ffc061;
      }
    }
    .character-two {
      position: absolute;
      right: -130px;
      z-index: -10;
    }

    .character-three {
      position: fixed;
      bottom: -20px;
      left: 0px;
      z-index: -20;
    }
  }
</style>
