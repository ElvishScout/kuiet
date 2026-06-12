<template>
  <div ref="divRef" class="kuiet-webview"></div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface WebviewInit {
  url?: string;
}

interface WebviewConfig {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  visibility?: boolean;
  url?: string;
}

const props = defineProps<{
  wid: string;
  invisible?: boolean;
  url?: string;
  persist?: boolean;
}>();

const divRef = ref<HTMLDivElement>();
let created = false;
let resizeObserver: ResizeObserver | null = null;

async function createWebview(wid: string, init: WebviewInit) {
  await invoke("create_webview", { wid, init });
}

async function configWebview(wid: string, config: WebviewConfig) {
  await invoke("config_webview", { wid, config });
}

async function destroyWebview(wid: string) {
  await invoke("destroy_webview", { wid });
}

let lastConfig: WebviewConfig = {};

function boundDiff() {
  if (!divRef.value) {
    return null;
  }

  const rect = divRef.value.getBoundingClientRect();
  const diff: WebviewConfig = {};

  if (rect.x !== lastConfig.x) diff.x = lastConfig.x = rect.x;
  if (rect.y !== lastConfig.y) diff.y = lastConfig.y = rect.y;
  if (rect.width !== lastConfig.width) diff.width = lastConfig.width = rect.width;
  if (rect.height !== lastConfig.height) diff.height = lastConfig.height = rect.height;

  return diff;
}

async function syncBounds() {
  const diff = boundDiff();

  if (diff && Object.keys(diff).length > 0) {
    await configWebview(props.wid, diff);
  }
}

onMounted(async () => {
  if (!divRef.value) return;

  await createWebview(props.wid, { url: props.url });
  created = true;

  await configWebview(props.wid, { ...boundDiff(), visibility: !props.invisible });

  resizeObserver = new ResizeObserver(syncBounds);
  resizeObserver.observe(divRef.value);
  window.addEventListener("resize", syncBounds);
});

watch(
  () => props.invisible,
  () => configWebview(props.wid, { visibility: !props.invisible }),
);

watch(
  () => props.url,
  () => configWebview(props.wid, { url: props.url }),
);

onBeforeUnmount(async () => {
  resizeObserver?.disconnect();
  resizeObserver = null;
  window.removeEventListener("resize", syncBounds);

  if (created) {
    if (props.persist) {
      await configWebview(props.wid, { visibility: false });
    } else {
      await destroyWebview(props.wid);
    }
    created = false;
  }
});
</script>

<style scoped>
.kuiet-webview {
  width: 100%;
  height: 100%;
}
</style>
