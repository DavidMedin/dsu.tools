<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import { ref, computed } from "vue";
import Page from "./components/Page.vue";
import ColorSlider from "./components/ColorSlider.vue";
import { Hct } from "@material/material-color-utilities";

// Controlled by the sliders.
// These defualt values are going to be overriden by the color sliders immediately.
const hct_color = ref([0, 0, 0]);

// Computed from hct_color whenever it changes.
// sRGB can be though of as simply RGB. I don't know much about color theory :(
const srgb_color_hex = computed(() => {
    return hctToSrgbHex(hct_color.value);
});

// The magic function that tells the ColorSlider component how to convert the color.
function hctToSrgbHex(hct) {
    let argb = Hct.from(hct[0], hct[1], hct[2]).argb.toString(16).slice(2);
    return argb;
}
</script>

<template>
    <Page>
        <p id="pregrid-text">Color Calculator</p>
        <div id="center">
            <div
                id="color-view"
                :style="{
                    backgroundColor: '#' + srgb_color_hex,
                }"
            ></div>

            <ColorSlider
                :color_space="hctToSrgbHex"
                :in_color="hct_color"
                :max_value="360"
                :variable_index="0"
                v-model="hct_color[0]"
            />

            <ColorSlider
                :color_space="hctToSrgbHex"
                :in_color="hct_color"
                :max_value="145"
                :variable_index="1"
                v-model="hct_color[1]"
            />

            <ColorSlider
                :color_space="hctToSrgbHex"
                :in_color="hct_color"
                :max_value="100"
                :variable_index="2"
                v-model="hct_color[2]"
            />
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
