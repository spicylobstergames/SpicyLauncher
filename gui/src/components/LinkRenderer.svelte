<script lang="ts">
  export let href = "";
  export let title = undefined;
</script>

<a {href} {title} target="_blank" rel="noreferrer"><slot /></a>
