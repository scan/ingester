import { zfd } from 'zod-form-data';
import { type z } from 'zod';
import { invoke } from '@tauri-apps/api/tauri';

const checkValidDirectory = async (path: string) =>
  await invoke<boolean>('directory_accessible', { path });

const schema = zfd.formData({
  sourceDir: zfd.text().refine(checkValidDirectory, { message: 'Invalid directory' }),
  rawTargetDir: zfd.text().refine(checkValidDirectory, { message: 'Invalid directory' }),
  jpegTargetDir: zfd.text().refine(checkValidDirectory, { message: 'Invalid directory' }),
  recursive: zfd.checkbox(),
});

export type IngestFormData = z.infer<typeof schema>;

export const validateFormData = async (formData: FormData) => await schema.safeParseAsync(formData);
