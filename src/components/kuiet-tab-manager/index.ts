import { Component, markRaw, reactive } from "vue";
import { TabManagerContext } from "./types";
import { nanoid } from "nanoid";

export class TabManager {
  context: TabManagerContext;

  constructor() {
    this.context = reactive({ tabs: [], index: -1, isHeaderReady: false });
  }

  create(component: Component, props: Record<string, any>) {
    const id = nanoid();
    this.context.tabs.push({ id, component: markRaw(component), props });
    return id;
  }

  createAndSelect(component: Component, props: Record<string, any>) {
    const id = this.create(component, props);
    this.context.index = this.context.tabs.length - 1;
    return id;
  }

  locate(id: string) {
    const index = this.context.tabs.findIndex((tab) => tab.id === id);
    return index !== -1 ? index : null;
  }

  select(index: number): void;
  select(id: string): void;
  select(indexOrId: number | string) {
    const index = typeof indexOrId === "string" ? (this.locate(indexOrId) ?? -1) : indexOrId;
    this.context.index = index;
  }

  remove(index: number): void;
  remove(id: string): void;
  remove(indexOrId: number | string) {
    const index = typeof indexOrId === "string" ? (this.locate(indexOrId) ?? -1) : indexOrId;
    this.context.tabs.splice(index, 1);
  }

  move(fromIndex: number, toIndex: number): void;
  move(fromId: string, toIndex: number): void;
  move(fromIndexOrId: number | string, toIndex: number) {
    const fromIndex = typeof fromIndexOrId === "string" ? (this.locate(fromIndexOrId) ?? -1) : fromIndexOrId;
    const [tab] = this.context.tabs.splice(fromIndex, 1);
    this.context.tabs.splice(fromIndex < toIndex ? toIndex - 1 : toIndex, 0, tab);
  }
}
