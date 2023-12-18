<script lang="ts">
  export let isLoading: boolean = false;

  $: disabled = isLoading;
</script>

<button type="submit" class="btn btn-primary w-full" class:btn-disabled={disabled} {disabled}>
  {#if disabled}
    <span class="loading loading-dots mx-2" />
  {:else}
    Start Ingesting!
  {/if}
</button>
