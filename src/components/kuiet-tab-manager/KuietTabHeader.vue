<template>
  <div class="kuiet-tab-header">
    <template v-for="tab in manager.context.tabs">
      <div :id="generateTeleportId(tab.id)" class="kuiet-tab-header__title"></div>
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
}

.kuiet-tab-header__title {
}
</style>
