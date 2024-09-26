<script setup>
import { onMounted, reactive, useTemplateRef, watch, computed } from "vue";

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

// Only used for the Handle's background color.
const in_color_hex_computed = computed(() => {
    return props.color_space(props.in_color);
});

// Some constants.
const slider_width = 500;
const slider_height = 30;
const handle_width = 24;

const initial_handle_pos = slider_width / 2;

let slider_trans_style = reactive({
    transform: handleTransformStyle(initial_handle_pos),
});

let pressed = false;

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

// useTemplateRef refers to this instance of the component's slider and canvas elements.
// If we used 'id' and 'document.getElementById' (or whatever) then all sliders and canvasas would be affected by this code.
const slider_el = useTemplateRef("slider-ref");
const canvas_el = useTemplateRef("color-slider-ref");
onMounted(() => {
    // Slider and Canvas are only defined after they are mounted into the DOM.

    let bound = slider_el.value.getBoundingClientRect();
    let left_bound_absolute = bound.left - handle_width / 2;

    slider_el.value.addEventListener("mousedown", (e) => {
        pressed = true;

        // If the window is resized, the left bound of the slider will change; update it.
        left_bound_absolute =
            slider_el.value.getBoundingClientRect().left - handle_width / 2;

        let new_x = e.x - left_bound_absolute - handle_width / 2;
        slider_trans_style = {
            transform: handleTransformStyle(new_x),
        };
        out_color_var.value = handlePosXToVariable(new_x);
    });
    window.addEventListener("mousemove", (e) => {
        if (!pressed) return;
        e.preventDefault();

        // Move the handle within the bounds of the slider.
        // slider space [0-slider_width]
        let new_x = e.x - left_bound_absolute - handle_width / 2;
        if (0 > new_x) {
            // if the handle is on the left of the slider...
            new_x = 0;
        } else if (slider_width < new_x) {
            // if it is on the right of the slider...
            new_x = slider_width;
        }

        // Actually move the slider.
        slider_trans_style = { transform: handleTransformStyle(new_x) };

        // Send the new color variable to the parent component.
        out_color_var.value = handlePosXToVariable(new_x);
    });

    window.addEventListener("mouseup", () => {
        pressed = false;
    });

    // Inital render and color output. Happens once.
    render();
    out_color_var.value = handlePosXToVariable(initial_handle_pos);
});

// Whenever the parent's color changes, re-render the slider.
watch(props.in_color, render);
</script>

<template>
    <div ref="slider-ref" id="slider">
        <canvas
            ref="color-slider-ref"
            id="color-slider"
            :width="slider_width"
            :height="slider_height"
        >
            There should be a color slide here
        </canvas>
        <svg id="handle" :style="slider_trans_style">
            <circle
                cx="12"
                cy="12"
                r="10"
                :fill="'#' + in_color_hex_computed"
            ></circle>
            <circle
                cx="12"
                cy="12"
                r="12"
                fill="none"
                stroke-width="3"
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
