<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import { provide } from "vue";
import { Hct } from "@material/material-color-utilities";
import Page from "./components/Page.vue";
import ColorRow from "./components/color-tool/ColorRow.vue";

// Component Hierarchy
// -------------------
// ColorRow
// |>ColorInput
//   |>ColorSlider
// |>ColorOutput

// Description of all color spaces we support.
// No other place in the code will know about any particular color space.
const color_spaces = [
    {
        name: "HCT",
        components: [
            { name: "Hue", range: [0, 360] },
            { name: "Chroma", range: [0, 120] },
            { name: "Tone", range: [0, 100] },
        ],
        description: "A neat little color space. Yuh",
        conversions: {
            fromRGBHex: (hex) => {
                let fmted_hex = Number("0x" + hex);
                let hct = Hct.fromInt(fmted_hex);
                return [hct.hue, hct.chroma, hct.tone];
            },
            toRGBHex: (hct) =>
                Hct.from(hct[0], hct[1], hct[2]).argb.toString(16).slice(2),
        },
    },
    {
        name: "RGB",
        components: [
            { name: "Red", range: [0, 255] },
            { name: "Green", range: [0, 255] },
            { name: "Blue", range: [0, 255] },
        ],
        description: "The color space of the web.",
        conversions: {
            fromRGBHex: (hex) => {
                let r = parseInt(hex.slice(0, 2), 16);
                let g = parseInt(hex.slice(2, 4), 16);
                let b = parseInt(hex.slice(4, 6), 16);
                return [r, g, b];
            },
            toRGBHex: (rgb) => {
                return rgb
                    .map((c) => Math.round(c).toString(16).padStart(2, "0"))
                    .join("");
            },
        },
    },
    {
        name: "HSV",
    },
    {
        name: "HSL",
    },
    {
        name: "OKlab",
    },
];
provide("ColorSpaces", color_spaces);
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
