// This file will be imported by vitamincalculator.html.

import './assets/main.css'

import { createApp } from 'vue'
import VitaminCalculator from './VitaminCalculator.vue'

// '#app' is a element selector that references the <div> in index.html.
// Our Vue code will be inserted into that div.
createApp(VitaminCalculator).mount('#vitamincalculator')