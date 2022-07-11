import { invoke } from '@tauri-apps/api';

async function get_item(key: string) {
  return invoke<string | null>('storage.get_item', { key });
}

async function set_item(key: string, value: string) {
  return invoke<void>('storage.set_item', { key, value });
}

export {
  get_item,
  set_item,
}
