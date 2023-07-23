import { invoke } from '@tauri-apps/api/tauri';
import type { IngestFormData } from './formHandler';

const copyFiles = (data: IngestFormData) =>
  invoke('copy_files', {
    sourcePath: data.sourceDir,
    rawPath: data.rawTargetDir,
    jpegPath: data.jpegTargetDir,
    recursive: data.recursive,
  });

export default copyFiles;
