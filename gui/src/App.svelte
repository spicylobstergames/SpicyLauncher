<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
</script>

<main>
  <div class="container">
    <div class="changelog">
      <h1>contetn</h1>
    </div>
    <Sidebar />
  </div>
</main>

<style lang="scss">
  @import url("https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap");
  @import "styles/app.scss";

  .container {
    display: flex;
    flex-direction: row;

    .changelog {
      flex: 7;
    }
    .sidebar {
      flex: 3;
      background-color: $color-white;
      height: 100vh;
    }
  }
</style>
