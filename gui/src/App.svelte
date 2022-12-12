<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import Changelog from "./components/Changelog.svelte";
  import GameTabs from "./components/GameTabs.svelte";
</script>
<!-- location.replace("http://localhost:8080") -->
<main>
  <div class="wrapper">
    <GameTabs />
    <div class="game-info">
      <Changelog />
      <Sidebar />
    </div>
  </div>
</main>

<style lang="scss">
  @import url("https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap");

  .wrapper {
    display: flex;
    flex-direction: column;
    height: 100vh;
    position: relative;
  }

  .game-info {
    height: 100%;
    overflow: hidden;
    display: flex;
    flex-direction: row;
  }
</style>
