<script lang="ts">
  import DirectoryChooser from './lib/DirectoryChooser.svelte';
  import copyFiles from './lib/copyFiles';
  import { validateFormData } from './lib/formHandler';

  let disabled = false;

  const handleSubmit = async (e: SubmitEvent) => {
    disabled = true;

    const formData = new FormData(e.target as HTMLFormElement);
    const validated = await validateFormData(formData);

    if (validated.success === false) {
      console.log(validated.error);
    } else {
      await copyFiles(validated.data);
    }

    disabled = false;
  };
</script>

<main class="container mx-auto">
  <form on:submit|preventDefault={handleSubmit}>
    <DirectoryChooser name="sourceDir">Source Directory</DirectoryChooser>
    <div class="form-control">
      <label class="label cursor-pointer">
        <span class="label-text">Recursive</span>
        <input type="checkbox" checked class="checkbox" name="recursive" />
      </label>
    </div>

    <DirectoryChooser name="rawTargetDir">Destination (RAW)</DirectoryChooser>
    <DirectoryChooser name="jpegTargetDir">Destination (JPEG)</DirectoryChooser>

    <div class="form-control my-5">
      <button class="btn btn-primary w-full" class:btn-disabled={disabled} {disabled}>
        {#if disabled}
          <span class="loading loading-dots mx-2" />
        {:else}
          Start Ingesting!
        {/if}
      </button>
    </div>
  </form>
</main>
