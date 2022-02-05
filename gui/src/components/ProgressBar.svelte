<script lang="ts">
  export let progress: number = 0;
</script>

<progress
  class={`nes-progress is-success progress`}
  value={progress}
  max="100"
/>

<style>
  .progress {
    width: 98%;
    margin-top: 10px;
  }
</style>
