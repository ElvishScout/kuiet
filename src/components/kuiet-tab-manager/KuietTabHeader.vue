<template>
  <div class="kuiet-tab-header">
    <template v-for="(tab, idx) in manager.context.tabs">
      <div class="kuiet-tab-header__title" @click="manager.select(idx)">
        <div class="title__content">
          <div :id="generateTeleportId(tab.id)"></div>
        </div>
        <div class="title__close-button"></div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import type { TabManager } from ".";
import { generateTeleportId } from "./utils";

const props = withDefaults(defineProps<{ manager: TabManager; persist?: boolean }>(), { persist: false });

onMounted(async () => {
  props.manager.context.isHeaderReady = true;
});

onUnmounted(() => {
  props.manager.context.isHeaderReady = false;
});
</script>

<style scoped>
.kuiet-tab-header {
  display: flex;
  flex-wrap: nowrap;
}

.kuiet-tab-header__title {
  display: flex;
  padding: 0 8px;
  align-items: center;
  min-width: 120px;
  max-width: 240px;
  border-radius: 8px;
  user-select: none;
  border: 1px solid lightgray;
}

.title__content {
  flex: 1 0 0;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title__close-button {
}

.kuiet-tab-header__title:hover {
  background-color: lightblue;
}
</style>
