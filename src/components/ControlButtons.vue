<template>
  <div class="control-buttons">
    <button class="button--minimize" @click="onMinimize">一</button>
    <button class="button--maximize" @click="onMaximizeToggle">{{ isMaximized ? "回" : "口" }}</button>
    <button class="button--close" @click="onClose">㐅</button>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";

const appWindow = getCurrentWindow();
const isMaximized = ref(false);

onMounted(async () => {
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
.control-buttons {
  display: flex;
}

.button--minimize,
.button--maximize,
.button--close {
  width: 48px;
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

.button--minimize:hover,
.button--maximize:hover {
  background: rgba(0, 0, 0, 0.12);
}

.button--minimize:active,
.button--maximize:active {
  background: rgba(0, 0, 0, 0.12);
}

.button--close:hover {
  background-color: #e81123;
  color: #fff;
}
</style>
