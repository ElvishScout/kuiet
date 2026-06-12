<template>
  <KuietTab>
    <template #title>{{ url }}</template>
  </KuietTab>
  <KuietWebview :wid="wid" :url="url"></KuietWebview>
</template>

<script setup lang="ts">
import { inject } from "vue";
import KuietTab from "../kuiet-tab-manager/KuietTab.vue";
import KuietWebview from "../KuietWebview.vue";

const tabId = inject<string>("kuietTabId");

if (tabId === undefined) {
  throw "tabId must be provided";
}

const wid = `webview-browser-${tabId}`;

const props = defineProps<{ url?: string }>();
</script>
