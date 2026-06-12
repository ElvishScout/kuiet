<template>
  <teleport v-if="manager.context.isHeaderReady" :to="`#${teleportId}`">
    <slot name="title"></slot>
  </teleport>
  <div class="kuiet-tab__panel">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { inject, onMounted, provide, Teleport, useId } from "vue";
import { generateTeleportId } from "./utils";
import type { TabManager } from ".";

const manager = inject<TabManager>("kuietTabManager");
const tabId = inject<string>("kuietTabId");

if (manager === undefined) {
  throw "kuietTabManager must be provided";
}

if (tabId === undefined) {
  throw "kuietTabId must be provided";
}

const teleportId = generateTeleportId(tabId);
</script>

<style scoped></style>
