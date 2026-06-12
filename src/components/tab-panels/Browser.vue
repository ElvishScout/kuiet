<template>
  <KuietTab>
    <template #title>{{ title }}</template>
  </KuietTab>
  <KuietWebview :wid="wid" :url="url"></KuietWebview>
</template>

<script setup lang="ts">
import { nanoid } from "nanoid";
import KuietTab from "../kuiet-tab-manager/KuietTab.vue";
import KuietWebview from "../KuietWebview.vue";

const wid = nanoid();

const props = defineProps<{ title?: string; url?: string }>();
</script>
