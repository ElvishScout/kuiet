import { platform as detectPlatform } from "@tauri-apps/plugin-os";

export const platform = detectPlatform();
