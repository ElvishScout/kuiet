<script setup lang="ts">
import { ref } from "vue";
import KuietWebview from "./components/KuietWebview.vue";

const showPersist = ref(true);
const showNopersist = ref(true);
</script>

<template>
  <div class="toolbar">
    <button @click="showPersist = !showPersist">
      {{ showPersist ? "Unmount" : "Mount" }} (persist)
    </button>
    <button @click="showNopersist = !showNopersist">
      {{ showNopersist ? "Unmount" : "Mount" }} (no persist)
    </button>
  </div>
  <div class="panels">
    <div class="panel">
      <div class="label">persist</div>
      <div v-if="showPersist" class="webview-box">
        <KuietWebview
          wid="persist-webview"
          url="https://example.com"
          persist
        />
      </div>
      <div v-else class="placeholder">unmounted — webview hidden but alive</div>
    </div>
    <div class="panel">
      <div class="label">no persist</div>
      <div v-if="showNopersist" class="webview-box">
        <KuietWebview
          wid="nopersist-webview"
          url="https://example.com"
        />
      </div>
      <div v-else class="placeholder">unmounted — webview destroyed</div>
    </div>
  </div>
</template>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
}

.toolbar {
  display: flex;
  gap: 12px;
  padding: 12px;
  background: #f0f0f0;
  border-bottom: 1px solid #ccc;
}

.toolbar button {
  padding: 6px 16px;
  cursor: pointer;
}

.panels {
  display: flex;
  flex: 1;
  height: calc(100% - 50px);
}

.panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #ccc;
}

.panel:last-child {
  border-right: none;
}

.label {
  padding: 8px 12px;
  font-weight: bold;
  background: #e8e8e8;
  border-bottom: 1px solid #ccc;
}

.webview-box {
  flex: 1;
  position: relative;
}

.placeholder {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #999;
  font-size: 14px;
}
</style>
