<template>
  <div class="kuiet-tab-view">
    <template v-for="(tab, idx) in manager.context.tabs">
      <div class="kuiet-tab-view__panel" v-show="idx === manager.context.index">
        <KuietTabProvider :tab-id="tab.id">
          <component :is="tab.component" v-bind="tab.props"></component>
        </KuietTabProvider>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { provide } from "vue";

import KuietTabProvider from "./KuietTabProvider.vue";
import type { TabManager } from ".";

const props = defineProps<{ manager: TabManager }>();

provide("kuietTabManager", props.manager);
</script>

<style scoped>
.kuiet-tab-view {
  width: 100%;
  height: 100%;
}

.kuiet-tab-view__panel {
  width: 100%;
  height: 100%;
}
</style>
