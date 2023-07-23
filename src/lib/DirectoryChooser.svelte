<script lang="ts">
  import { open } from '@tauri-apps/api/dialog';
  import { pictureDir } from '@tauri-apps/api/path';
    import { onMount } from 'svelte';

  export let name: string;

  let chosenPath = '';
  const id = `id-directory-chooser-${name}`;
  const storageKey = `form-${name}`;

  const handleOpenClick = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: await pictureDir(),
    });
    
    chosenPath = Array.isArray(selected) ? selected[0] : selected;

    localStorage.setItem(storageKey, chosenPath);
  };

  onMount(() => {
    const savedValue = localStorage.getItem(storageKey);
    if (typeof savedValue === "string") {
        chosenPath = savedValue;
    }
  })
</script>

<div class="form-control w-full max-w-xs">
  <label class="label" for={id}><slot /></label>
  <div class="join w-full">
    <input
      type="text"
      class="input input-bordered input-primary w-full max-w-xs join-item"
      {id}
      {name}
      readonly
      bind:value={chosenPath}
    />
    <button type="button" class="join-item btn btn-primary" on:click={handleOpenClick}
      >Choose...</button
    >
  </div>
</div>
