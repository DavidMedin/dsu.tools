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

// TODO: deduplicate functions.

defaults.precision = 2;
ColorSpace.register(HCT);
// ColorSpace.register(A98RGB);
// ColorSpace.register(HSL);
// ColorSpace.register(HSLuv);
// ColorSpace.register(HSV);
// ColorSpace.register(OKLab);
// ColorSpace.register(OKLCH);
// ColorSpace.register(P3);
ColorSpace.register(sRGB_Linear);
ColorSpace.register(OKLCH);

console.log(ColorSpace.all);

// Description of all color spaces we support.
// No other place in the code will know about any particular color space.
// const color_spaces = [
//     {
//         name: "HCT",
//         components: [
//             { name: "Hue", range: [0, 360] },
//             { name: "Chroma", range: [0, 120] },
//             { name: "Tone", range: [0, 100] },
//         ],
//         description: "A neat little color space. Yuh",
//         conversions: {
//             fromRGBHex: (hex) => {
//                 let fmted_hex = Number("0x" + hex);
//                 let hct = Hct.fromInt(fmted_hex);
//                 return [hct.hue, hct.chroma, hct.tone];
//             },
//             toRGBHex: (hct) =>
//                 Hct.from(hct[0], hct[1], hct[2]).argb.toString(16).slice(2),
//         },
//     },
//     {
//         name: "RGB",
//         components: [
//             { name: "Red", range: [0, 255] },
//             { name: "Green", range: [0, 255] },
//             { name: "Blue", range: [0, 255] },
//         ],
//         description: "The color space of the web.",
//         conversions: {
//             fromRGBHex: (hex) => {
//                 let r = parseInt(hex.slice(0, 2), 16);
//                 let g = parseInt(hex.slice(2, 4), 16);
//                 let b = parseInt(hex.slice(4, 6), 16);
//                 return [r, g, b];
//             },
//             toRGBHex: (rgb) => {
//                 return rgb
//                     .map((c) => Math.round(c).toString(16).padStart(2, "0"))
//                     .join("");
//             },
//         },
//     },
//     {
//         name: "HSV",
//         components: [
//             { name: "Hue", range: [0, 360] },
//             { name: "Saturation", range: [0, 100] },
//             { name: "Value", range: [0, 100] },
//         ],
//         description: "Ubiquitousskjskdf",
//         conversion: {
//             fromRGBHex: (hex) => {},
//             toRGBHex: (rgb) => {},
//         },
//     },
//     {
//         name: "HSL",
//     },
//     {
//         name: "OKlab",
//     },
// ];

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
