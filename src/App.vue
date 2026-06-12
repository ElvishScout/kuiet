<template>
  <div class="container">
    <div class="title-bar" data-tauri-drag-region>
      <KuietTabHeader class="tab-header" :manager="manager" />
      <ControlButtons v-if="platform !== 'macos'" class="control-buttons" />
    </div>
    <KuietTabView class="tab-view" :manager="manager" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";

import KuietTabHeader from "./components/kuiet-tab-manager/KuietTabHeader.vue";
import KuietTabView from "./components/kuiet-tab-manager/KuietTabView.vue";
import Browser from "./components/tab-panels/Browser.vue";

import { TabManager } from "./components/kuiet-tab-manager/index.ts";
import ControlButtons from "./components/ControlButtons.vue";
import { platform } from "./utils.ts";

const manager = new TabManager();

onMounted(async () => {
  manager.createAndSelect(Browser, { url: "https://example.com" });
});
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
}

.title-bar {
  display: flex;
  padding: 4px 8px 4px 8px;
  height: 36px;
  box-sizing: border-box;
}

[data-platform="macos"] .title-bar {
  padding-left: 108px;
}

.tab-header {
  width: fit-content;
  min-width: 0;
  overflow: hidden;
}

.tab-header:not(:last-child) {
  margin-right: 48px;
}

.control-buttons {
  margin: -4px -8px -4px auto;
}

.tab-view {
  flex: 1 0 0;
}
</style>
