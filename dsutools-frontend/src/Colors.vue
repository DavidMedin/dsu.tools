<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import { ref } from "vue";
import Page from "./components/Page.vue";
import ColorSlider from "./components/ColorSlider.vue";
import { Hct } from "@material/material-color-utilities";

// let colorView = null;
// function onColor(color) {
//     if (colorView == null) {
//         colorView = document.getElementById("color-view");
//     }
//     colorView.style.backgroundColor = `rgb(${color[0]}, ${color[1]}, ${color[2]})`;
// }

const color = ref({ hue: 0, chroma: 122, tone: 50 })
const hueOutColor = ref('green')

function hctToSrgb(hct) {
    let argb = Hct.from(hct.hue, hct.chroma, hct.tone)
        .argb.toString(16)
        .slice(2);
    return argb;
}
</script>

<template>
    <Page>
        <p id="pregrid-text">Color Calculator</p>
        <div id="center">
            <div id="color-view" :style="{'background-color': hueOutColor}"></div>
            <ColorSlider :color_space="hctToSrgb" :in_color="color" v-model:out_color="hueOutColor" />
        </div>
    </Page>
</template>

<style scoped>
#pregrid-text {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    text-align: center;
}
#color-view {
    width: 5rem;
    height: 5rem;
    /* background-color: red; */
    border-radius: 1rem;
    margin: 1rem;
}

#center {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>
