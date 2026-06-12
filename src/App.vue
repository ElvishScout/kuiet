<template>
  <div class="container">
    <div class="title-bar" data-tauri-drag-region>
      <KuietTabHeader class="tab-header" :manager="manager" />
      <div class="title-bar__buttons">
        <button class="win-btn win-btn--min" aria-label="Minimize" @click="onMinimize"></button>
        <button
          class="win-btn"
          :class="isMaximized ? 'win-btn--restore' : 'win-btn--max'"
          :aria-label="isMaximized ? 'Restore' : 'Maximize'"
          @click="onMaximizeToggle"
        ></button>
        <button class="win-btn win-btn--close" aria-label="Close" @click="onClose"></button>
      </div>
    </div>
    <KuietTabView class="tab-view" :manager="manager" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

import KuietTabHeader from "./components/kuiet-tab-manager/KuietTabHeader.vue";
import KuietTabView from "./components/kuiet-tab-manager/KuietTabView.vue";
import Browser from "./components/tab-panels/Browser.vue";

import { TabManager } from "./components/kuiet-tab-manager/index.ts";

const manager = new TabManager();
const appWindow = getCurrentWindow();
const isMaximized = ref(false);

onMounted(async () => {
  manager.createAndSelect(Browser, { url: "https://example.com" });

  isMaximized.value = await appWindow.isMaximized();

  await appWindow.onResized(() => {
    appWindow.isMaximized().then((v) => (isMaximized.value = v));
  });
});

function onMinimize() {
  appWindow.minimize();
}

function onMaximizeToggle() {
  appWindow.toggleMaximize().then(() => {
    appWindow.isMaximized().then((v) => (isMaximized.value = v));
  });
}

function onClose() {
  appWindow.close();
}
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
  padding: 4px 12px 4px 12px;
  height: 36px;
  box-sizing: border-box;
}

[data-platform="macos"] .title-bar {
  padding-left: 108px;
}

.tab-header {
  margin-right: 32px;
  width: fit-content;
  min-width: 0;
  overflow: hidden;
}

[data-platform="macos"] .tab-header {
  margin-right: 0;
}

.title-bar__buttons {
  margin: -4px -12px -4px auto;
  display: flex;
  gap: 1px;
}

[data-platform="macos"] .title-bar__buttons {
  display: none;
}

.win-btn {
  margin: -4px 0;
  width: 52px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  position: relative;
  transition: background-color 0.1s;
  border: none;
  outline: none;
  box-sizing: border-box;
  font: inherit;
  color: inherit;
  background: transparent;
  padding: 0;
}

.win-btn::-moz-focus-inner {
  border: 0;
}

.win-btn:hover {
  background: rgba(0, 0, 0, 0.12);
}

.win-btn:active {
  background: rgba(0, 0, 0, 0.22);
}

/* Minimize — horizontal line */
.win-btn--min::after {
  content: "";
  display: block;
  width: 12px;
  height: 1px;
  background: currentColor;
}

/* Maximize — outlined square */
.win-btn--max::after {
  content: "";
  display: block;
  width: 10px;
  height: 10px;
  border: 1px solid currentColor;
}

/* Restore — two overlapping squares */
.win-btn--restore::before,
.win-btn--restore::after {
  content: "";
  position: absolute;
  display: block;
  width: 8px;
  height: 8px;
  border: 1px solid currentColor;
  background: transparent;
}
.win-btn--restore::before {
  top: 16px;
  left: 20px;
  background: currentColor;
  opacity: 0.25;
}
.win-btn--restore::after {
  top: 18px;
  left: 22px;
}

/* Close — X shape */
.win-btn--close::before,
.win-btn--close::after {
  content: "";
  position: absolute;
  display: block;
  width: 14px;
  height: 1px;
  background: currentColor;
  transform: rotate(45deg);
}
.win-btn--close::after {
  transform: rotate(-45deg);
}

.win-btn--close:hover {
  background-color: #e81123;
  color: #fff;
}

.tab-view {
  flex: 1 0 0;
}
</style>
