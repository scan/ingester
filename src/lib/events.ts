export const EVENT_COPY_PROGRESS_DONE = 'copy_progress_done';
export const EVENT_COPY_START = 'copy_start';
export const EVENT_COPY_FINISHED = 'copy_finished';

export interface CopyProgressEvent {
    fileName: string;
    source: string;
    destination: string;
}
