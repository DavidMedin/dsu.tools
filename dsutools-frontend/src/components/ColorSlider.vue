<script setup>
import { onMounted } from "vue";
import { Hct } from "@material/material-color-utilities";

const slider_width = 500;
const slider_height = 30;
const handle_height = 24;

let pressed = false;
let cursorOffset = 0;

function hexToBytes(hex) {
    let bytes = [];
    for (let c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}

function hctToSrgb(hct) {
    let argb = Hct.from(hct.hue, hct.chroma, hct.tone)
        .argb.toString(16)
        .slice(2);
    let color = hexToBytes(argb);
    color.push(255);
    return color;
}

function handleTransformStyle(x) {
    return `translate(${x - slider_width}px,0px)`;
    // return `translate(${x}px,${slider_height / 2 - handle_height / 2}px)`;
}

onMounted(() => {
    // Set the slider handlers
    const handle_el = document.getElementById("handle");
    const slider_el = document.getElementById("color-slider");

    let bound = slider_el.getBoundingClientRect();
    let left_bound_absolute = bound.left;
    let right_bound_relative = slider_width - 24; // the SVG handle for the slider is 24px wide.

    handle_el.addEventListener("mousedown", (e) => {
        pressed = true;
        // slider.style.cursor = "grabbing";
        cursorOffset = e.x - handle_el.getBoundingClientRect().left;

        // If the winodw is resized, the left bound of the slider will change; update it.
        left_bound_absolute = slider_el.getBoundingClientRect().left;
    });
    window.addEventListener("mousemove", (e) => {
        if (!pressed) return;
        e.preventDefault();

        // slider space [0-slider_width]
        let new_x = e.x - cursorOffset - left_bound_absolute;
        if (0 > new_x) {
            new_x = 0;
        } else if (right_bound_relative < new_x) {
            new_x = right_bound_relative;
        }

        handle_el.style.transform = handleTransformStyle(new_x);
    });

    window.addEventListener("mouseup", () => {
        pressed = false;
    });

    // render the hue color slider
    const canvas = document.getElementById("color-slider");
    if (canvas.getContext) {
        const ctx = canvas.getContext("2d");

        let img_arr = new Uint8ClampedArray(slider_width * slider_height * 4);
        for (let i = 0; i < slider_width; i++) {
            let hct = { hue: (i / slider_width) * 360, chroma: 122, tone: 50 };
            let color = hctToSrgb(hct);
            for (let j = 0; j < slider_height; j++) {
                img_arr.set(color, (j * slider_width + i) * 4);
            }
        }

        let img_data = new ImageData(img_arr, slider_width, slider_height, {
            colorSpace: "srgb",
        });
        ctx.putImageData(img_data, 0, 0);
    }
});
</script>

<template>
    <!-- <p>uh</p> -->
    <div id="slider">
        <canvas id="color-slider" :width="slider_width" :height="slider_height">
            There should be a color slide here
        </canvas>
        <svg
            id="handle"
            :style="{
                transform: handleTransformStyle(0),
            }"
        >
            <circle
                cx="12"
                cy="12"
                r="12"
                fill="none"
                stroke-width="2"
                stroke="#000"
            ></circle>
            <circle
                cx="12"
                cy="12"
                r="10"
                fill="none"
                stroke-width="2"
                stroke="#fff"
            ></circle>
        </svg>
    </div>
</template>

<style scoped>
#slider {
    position: relative;
    display: flex;
    flex-direction: row;
    align-items: center;
}
/* #color-slider {
    position: absolute;
    left: 0;
    top: 0;
} */
#handle {
    /* transform: translate(0px, 0px); */
    will-change: transform;
    width: 24px;
    height: 24px;
    overflow: visible;
}
</style>
