<script lang="ts">
  export let progress: number = 0;
</script>

<div class="progress-wrapper">
  <progress
    class={`nes-progress is-success progress`}
    value={progress}
    max="100"
  />

  <span>%{progress.toFixed(2)}</span>
</div>

<style lang="scss">
  .progress-wrapper {
    position: relative;

    span {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
    }
  }

  .progress {
    width: 98%;
    margin-top: 10px;
  }
</style>
