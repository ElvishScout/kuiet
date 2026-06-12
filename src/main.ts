import "./style.css";

import { createApp } from "vue";
import App from "./App.vue";
import { platform } from "./utils.ts";

const root = document.querySelector("#app") as HTMLElement;
root.dataset.platform = platform;

createApp(App).mount(root);
