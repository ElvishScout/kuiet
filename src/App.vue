<template>
  <div class="container">
    <div class="tab-header-wrap" data-tauri-drag-region>
      <KuietTabHeader class="tab-header" :manager="manager" />
    </div>
    <KuietTabView class="tab-view" :manager="manager" />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";

import KuietTabHeader from "./components/kuiet-tab-manager/KuietTabHeader.vue";
import KuietTabView from "./components/kuiet-tab-manager/KuietTabView.vue";
import Browser from "./components/tab-panels/Browser.vue";

import { TabManager } from "./components/kuiet-tab-manager/index.ts";

const manager = new TabManager();

onMounted(() => {
  manager.createAndSelect(Browser, { url: "https://example.com" });
  // manager.createAndSelect(Browser, { url: "https://www.bing.com" });
});
</script>

<style>
.container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
}

.tab-header-wrap {
  padding: 8px 12px 8px 108px;
  height: 42px;
  box-sizing: border-box;
}

.tab-header {
  width: fit-content;
  height: 100%;
}

.tab-view {
  flex: 1 0 0;
}
</style>
