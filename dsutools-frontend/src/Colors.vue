<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import Page from "./components/Page.vue";
import ColorSlider from "./components/ColorSlider.vue";
import { onMounted } from "vue";

let colorView = null;
function onColor(color) {
    if (colorView == null) {
        colorView = document.getElementById("color-view");
    }
    colorView.style.backgroundColor = `rgb(${color[0]}, ${color[1]}, ${color[2]})`;
}
</script>

<template>
    <Page>
        <p id="pregrid-text">Color Calculator</p>
        <div id="center">
            <div id="color-view"></div>
            <ColorSlider @color="onColor" />
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
    background-color: red;
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
