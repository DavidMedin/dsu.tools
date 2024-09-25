// This file will be imported by index.html.

import "./assets/main.css";

import { createApp } from "vue";
import Colors from "./Colors.vue";

// '#app' is a element selector that references the <div> in index.html.
// Our Vue code will be inserted into that div.
createApp(Colors).mount("#app");
