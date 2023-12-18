<script lang="ts">
  import { onMount } from 'svelte';
  import { writable, type Writable } from 'svelte/store';
  import { listen, type Event } from '@tauri-apps/api/event';
  import {
    type CopyProgressEvent,
    EVENT_COPY_PROGRESS_DONE,
    EVENT_COPY_START,
    EVENT_COPY_FINISHED,
  } from './events';

  const lines: Writable<Array<string>> = writable([]);

  function scrollIntoView(className) {
    const el = document.querySelector(`.${className}`);
    if (!el) return;
    el.scrollIntoView({
      behavior: 'smooth',
    });
  }

  onMount(() => {
    let unlisten: () => void | undefined;

    const mount = async () => {
      const unlistenStart = await listen(EVENT_COPY_START, () => {
        lines.set(['Starting to copy files...']);
      });

      const unlistenFinish = await listen(EVENT_COPY_FINISHED, () => {
        lines.update((prev) => [...prev, 'Finished copying files.']);
      });

      const unlistenProgress = await listen(
        EVENT_COPY_PROGRESS_DONE,
        (event: Event<CopyProgressEvent>) => {
          const { file_name: filename } = event.payload;

          lines.update((prev) => [...prev, `Copied ${filename}`]);

          requestAnimationFrame(() => scrollIntoView('last-item'));
        }
      );

      unlisten = () => {
        unlistenStart();
        unlistenFinish();
        unlistenProgress();
      };
    };

    mount();

    return () => {
      if (typeof unlisten === 'function') {
        unlisten();
      }
    };
  });
</script>

<div class="overflow-y-scroll font-mono text-sm max-h-[9rem]">
  {#each $lines as line, index (line)}
    <p class="text-green-500" class:last-item={index === $lines.length - 1}>{line}</p>
  {/each}
</div>
