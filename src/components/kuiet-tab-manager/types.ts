import type { Component } from "vue";

export interface TabItem {
  id: string;
  component: Component;
  props: Record<string, any>;
}

export interface TabManagerContext {
  tabs: TabItem[];
  index: number;
  isHeaderReady: boolean;
}
