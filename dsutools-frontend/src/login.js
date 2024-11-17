// This file will be imported by index.html.

import './assets/main.css'

import { createApp } from 'vue'
import Login from './Login.vue'

// '#app' is a element selector that references the <div> in index.html.
// Our Vue code will be inserted into that div.
createApp(Login).mount('#login')