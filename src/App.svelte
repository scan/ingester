<script lang="ts">
    import { writable } from 'svelte/store';
  import DirectoryChooser from './lib/DirectoryChooser.svelte';
    import IngestSubmitButton from './lib/IngestSubmitButton.svelte';
    import ProgressLogger from './lib/ProgressLogger.svelte';
  import copyFiles from './lib/copyFiles';
  import { validateFormData } from './lib/formHandler';

  let isProcessing = writable(false);

  const handleSubmit = async (e: SubmitEvent) => {
    isProcessing.set(true);

    const formData = new FormData(e.target as HTMLFormElement);
    const validated = await validateFormData(formData);

    if (validated.success === false) {
      console.log(validated.error);
    } else {
      await copyFiles(validated.data);
    }

    isProcessing.set(false);
  };
</script>

<main class="container mx-auto md:px-5 h-screen">
  <form on:submit|preventDefault={handleSubmit} class="h-full min-h-screen flex flex-col pt-10">
    <div class="grow-0 shrink-0 flex flex-col">
      <DirectoryChooser name="sourceDir">Source Directory</DirectoryChooser>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text">Recursive</span>
          <input type="checkbox" checked class="checkbox" name="recursive" />
        </label>
      </div>

      <DirectoryChooser name="rawTargetDir">Destination (RAW)</DirectoryChooser>
      <DirectoryChooser name="jpegTargetDir">Destination (JPEG)</DirectoryChooser>
      <DirectoryChooser name="movieTargetDir">Destination (Movie)</DirectoryChooser>
    </div>

    <div class="w-full grow-0 shrink-0">
      <div class="form-control py-5">
        <IngestSubmitButton isLoading={$isProcessing} />
      </div>
    </div>

    <div class="w-full grow self-end pb-5">
      <ProgressLogger />
    </div>
  </form>
</main>
