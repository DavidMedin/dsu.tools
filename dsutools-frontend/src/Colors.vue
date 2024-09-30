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
        components: ["Hue", "Chroma", "Tone"],
        description: "A neat little color space. Yuh",
        conversions: {
            fromRGBHex: () => {
                let hct = Hct.fromInt(srgb);
                return [hct.hue, hct.chroma, hct.tone];
            },
            toRGBHex: (hct) =>
                Hct.from(hct[0], hct[1], hct[2]).argb.toString(16).slice(2),
        },
    },
    {
        name: "RGB",
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
