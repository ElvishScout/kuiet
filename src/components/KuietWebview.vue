<template>
  <div ref="divRef" class="kuiet-webview"></div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getAllWebviews } from "@tauri-apps/api/webview";

interface WebviewInit {
  url?: string;
}

interface WebviewConfig {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  visibility?: boolean;
  focused?: boolean;
  url?: string;
}

interface WebviewStatus {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  visibility?: boolean;
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

let lastStatus: WebviewStatus = {};

function statusDiff(options?: Partial<Record<keyof WebviewStatus, boolean>>) {
  if (!divRef.value) {
    return null;
  }

  options ??= {
    x: true,
    y: true,
    width: true,
    height: true,
    visibility: true,
  };

  const status: WebviewStatus = {};

  if (options.x || options.y || options.width || options.height) {
    const rect = divRef.value.getBoundingClientRect();

    if (options.x) status.x = rect.x;
    if (options.y) status.y = rect.y;
    if (options.width) status.width = rect.width;
    if (options.height) status.height = rect.height;
  }

  if (options.visibility) {
    let visibility = !props.invisible;

    if (visibility) {
      if ("checkVisibility" in Element.prototype) {
        visibility = divRef.value.checkVisibility({ visibilityProperty: true });
      } else {
        const styles = divRef.value.computedStyleMap();
        visibility =
          !divRef.value.hidden && styles.get("display") !== "none" && styles.get("visibility") !== "invisible";
      }
    }

    status.visibility = visibility;
  }

  const diff: WebviewStatus = {};

  for (const _key in status) {
    const key = _key as keyof WebviewStatus;

    if (status[key] !== lastStatus[key]) {
      diff[key] = status[key] as any;
    }
  }

  return diff;
}

async function syncConfig(options?: Partial<Record<keyof WebviewStatus, boolean>>) {
  const diff = statusDiff(options);

  if (diff && Object.keys(diff).length > 0) {
    Object.assign(lastStatus, diff);
    await configWebview(props.wid, diff);
  }
}

async function syncBoundsAndVisibility() {
  const webviews = await getAllWebviews();
  console.log(webviews);

  await syncConfig({ x: true, y: true, width: true, height: true, visibility: true });
}

onMounted(async () => {
  if (!divRef.value) return;

  await createWebview(props.wid, { url: props.url });
  created = true;

  await syncConfig();

  resizeObserver = new ResizeObserver(syncBoundsAndVisibility);
  resizeObserver.observe(divRef.value);
  window.addEventListener("resize", syncBoundsAndVisibility);
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
  window.removeEventListener("resize", syncBoundsAndVisibility);

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
