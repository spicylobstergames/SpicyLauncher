<script>
  import { currentGame } from "../currentGame";
</script>

<div class="row">
  <h1 class="heading">Games:</h1>
  <button
    type="button"
    class="nes-btn is-warning btn"
    class:active={$currentGame === "jumpy"}
    on:click={() => {
      $currentGame = "jumpy";
    }}
  >
    Jumpy
  </button>
  <button
    type="button"
    class="nes-btn is-warning btn"
    class:active={$currentGame === "punchy"}
    on:click={() => {
      $currentGame = "punchy";
    }}
  >
    Punchy
  </button>
</div>

<style>
  .row {
    display: flex;
    padding-left: 1em;
    align-items: center;
  }

  .active {
    filter: brightness(1.2);
  }

  .btn {
    padding: 0em;
    margin-right: 1em;
  }
</style>
