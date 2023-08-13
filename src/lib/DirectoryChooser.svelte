<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { pictureDir } from '@tauri-apps/api/path';
  import { onMount } from 'svelte';

  export let name: string;

  let chosenPath = '';
  let opened = false;
  const id = `id-directory-chooser-${name}`;
  const storageKey = `form-${name}`;

  const handleOpenClick = async () => {
    opened = true;
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await pictureDir(),
    });
    opened = false;

    if (selected.length === 0) {
      return;
    }

    chosenPath = Array.isArray(selected) ? selected[0] : selected;

    localStorage.setItem(storageKey, chosenPath);
  };

  onMount(() => {
    const savedValue = localStorage.getItem(storageKey);
    if (typeof savedValue === 'string') {
      chosenPath = savedValue;
    }
  });
</script>

<div class="form-control w-full">
  <label class="label" for={id}><slot /></label>
  <div class="join w-full">
    <input
      type="text"
      class="input input-bordered input-primary w-full join-item"
      {id}
      {name}
      readonly
      bind:value={chosenPath}
    />
    <button
      type="button"
      class="join-item btn btn-primary"
      class:disabled={opened}
      disabled={opened}
      on:click={handleOpenClick}
    >
      {#if opened}
        <span class="loading loading-dots mx-2" />
      {:else}
        Choose...
      {/if}
    </button>
  </div>
</div>
