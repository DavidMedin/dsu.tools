<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import { provide } from "vue";
import { Hct } from "@material/material-color-utilities";
import {
    defaults,
    ColorSpace,
    HCT,
    A98RGB,
    HSL,
    HSLuv,
    HSV,
    OKLab,
    OKLCH,
    P3,
    sRGB,
    sRGB_Linear,
} from "colorjs.io/fn";
import Page from "./components/Page.vue";
import ColorRow from "./components/color-tool/ColorRow.vue";

// Component Hierarchy
// -------------------
// ColorRow
// |>ColorInput
//   |>ColorSlider
// |>ColorOutput

defaults.precision = 2;
ColorSpace.register(HCT);
ColorSpace.register(A98RGB);
ColorSpace.register(HSL);
ColorSpace.register(HSLuv);
ColorSpace.register(HSV);
ColorSpace.register(OKLab);
ColorSpace.register(OKLCH);
ColorSpace.register(P3);
ColorSpace.register(sRGB_Linear);
ColorSpace.register(OKLCH);

console.log(ColorSpace.all);

provide("ColorSpaces", ColorSpace.all);
</script>

<template>
    <Page>
        <p id="pregrid-text">Color Calculator</p>
        <div id="center">
            <ColorRow />
        </div>
    </Page>
</template>

<style scoped>
#center {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>
