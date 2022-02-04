<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
</script>

<main>
  <div class="container">
    <div class="changelog" />
    <Sidebar />
  </div>
</main>

<style lang="scss">
  @import url("https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap");

  .container {
    display: flex;
    flex-direction: row;

    .changelog {
      flex: 5;
    }
  }
</style>
