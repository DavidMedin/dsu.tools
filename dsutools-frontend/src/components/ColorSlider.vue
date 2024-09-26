<script setup>
import { onMounted, reactive, useTemplateRef, watch } from "vue";

// Input/Output Model for the Color Slider:
// {
// [parameter] colorSpace : Function(colorspace-color) -> sRGB-color-hex
// [input] in_color : Array (color)
// [input] max_value : Number
// [input] variable_index : Number
// [output] out_variable : Number
// }
const props = defineProps({
    color_space: {
        type: Function,
        required: true,
    },
    in_color: {
        type: Array,
        required: true,
    },
    max_value: {
        type: Number,
        required: true,
    },
    variable_index: {
        type: Number,
        required: true,
    },
});
const out_color_var = defineModel({ required: true });

const slider_width = 500;
const slider_height = 30;
const handle_width = 24;

let slider_trans_style = reactive({
    transform: handleTransformStyle(0),
});

let pressed = false;
// let cursorOffset = 0;

function hexToBytes(hex) {
    let bytes = [];
    for (let c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}
function handlePosXToVariable(x) {
    return (x / slider_width) * props.max_value;
}
function handlePosXToHexColor(x) {
    // Create the color in the user's color space.
    // It uses the color provided in props.in_color as the base,
    // then modifies the variable at props.variable_index with the
    // value the slider represents.
    let color_space_color = props.in_color.with(
        props.variable_index,
        handlePosXToVariable(x),
    );

    // Convert the color from the user's color space to sRGB as hex.
    let hex_color = props.color_space(color_space_color);
    return hex_color;
}

function handleTransformStyle(x) {
    return `translate(${x - slider_width - handle_width / 2}px,0px)`;
}

function render(new_color, old_color) {
    // render the hue color slider
    if (canvas_el.value.getContext) {
        const ctx = canvas_el.value.getContext("2d");

        let img_arr = new Uint8ClampedArray(slider_width * slider_height * 4);
        for (let i = 0; i < slider_width; i++) {
            let hex_color = handlePosXToHexColor(i);
            let color = hexToBytes(hex_color);
            color.push(255);
            for (let j = 0; j < slider_height; j++) {
                img_arr.set(color, (j * slider_width + i) * 4);
            }
        }

        let img_data = new ImageData(img_arr, slider_width, slider_height, {
            colorSpace: "srgb",
        });
        ctx.putImageData(img_data, 0, 0);
    }
}

const handle_el = useTemplateRef("handle-ref");
const slider_el = useTemplateRef("slider-ref");
const canvas_el = useTemplateRef("color-slider-ref");
onMounted(() => {
    // Set the slider handlers

    let bound = slider_el.value.getBoundingClientRect();
    let left_bound_absolute = bound.left - handle_width / 2;
    let right_bound_relative = slider_width; // the SVG handle for the slider is 24px wide.

    slider_el.value.addEventListener("mousedown", (e) => {
        pressed = true;
        // slider.style.cursor = "grabbing";
        // cursorOffset = e.x - handle_el.value.getBoundingClientRect().left;

        let new_x = e.x - left_bound_absolute - handle_width / 2;
        slider_trans_style = {
            transform: handleTransformStyle(new_x),
        };
        out_color_var.value = handlePosXToVariable(new_x);
        // If the window is resized, the left bound of the slider will change; update it.
        left_bound_absolute =
            slider_el.value.getBoundingClientRect().left - handle_width / 2;
    });
    window.addEventListener("mousemove", (e) => {
        if (!pressed) return;
        e.preventDefault();

        // slider space [0-slider_width]
        let new_x = e.x - left_bound_absolute - handle_width / 2;
        if (0 > new_x) {
            new_x = 0;
        } else if (right_bound_relative < new_x) {
            new_x = right_bound_relative;
        }

        slider_trans_style = { transform: handleTransformStyle(new_x) };

        // out_color.value = handlePosXToHexColor(new_x);
        out_color_var.value = handlePosXToVariable(new_x);
    });

    window.addEventListener("mouseup", () => {
        pressed = false;
    });

    render();
    out_color_var.value = handlePosXToVariable(0);
});

watch(props.in_color, render);
</script>

<template>
    <!-- <p>uh</p> -->
    <div ref="slider-ref" id="slider">
        <canvas
            ref="color-slider-ref"
            id="color-slider"
            :width="slider_width"
            :height="slider_height"
        >
            There should be a color slide here
        </canvas>
        <svg ref="handle-ref" id="handle" :style="slider_trans_style">
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
    margin: 1rem;
}
#color-slider {
    border-radius: 0.5rem;
}
#handle {
    will-change: transform;
    width: 24px;
    height: 24px;
    overflow: visible;
}
</style>
