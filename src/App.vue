<template>
  <div class="container">
    <KuietTabHeader :manager="manager" />
    <KuietTabView class="tab-view" :manager="manager" />
  </div>
</template>

<script setup lang="ts">
import { markRaw, onMounted, ref } from "vue";
import { nanoid } from "nanoid";

import KuietWebview from "./components/KuietWebview.vue";
import KuietTabHeader from "./components/kuiet-tab-manager/KuietTabHeader.vue";
import KuietTabView from "./components/kuiet-tab-manager/KuietTabView.vue";
import Browser from "./components/tab-panels/Browser.vue";

import { TabManager } from "./components/kuiet-tab-manager/index.ts";

const manager = new TabManager();

onMounted(() => {
  manager.createAndSelect(Browser, { title: "hello", url: "https://example.com" });
});
</script>

<style>
.container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
}

.tab-view {
  flex: 1 0 0;
}
</style>
