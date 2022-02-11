<script lang="ts">
  import Sidebar from "./components/Sidebar.svelte";
  import Changelog from "./components/Changelog.svelte";
</script>
<!-- location.replace("http://localhost:8080") -->
<main>
  <div class="container">
    <Changelog />
    <Sidebar />
  </div>
</main>

<style lang="scss">
  @import url("https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap");

  .container {
    display: flex;
    flex-direction: row;
  }
</style>
