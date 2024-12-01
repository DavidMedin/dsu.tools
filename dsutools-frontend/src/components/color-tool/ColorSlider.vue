<script setup>
import { onMounted, useTemplateRef, watch, computed, ref } from "vue";

// Input/Output Model for the Color Slider:
// {
// [parameter] colorSpace : Function(colorspace-color) -> sRGB-color-hex
// [input] (model) color : Array (color)
// [input] max_value : Number
// [input] variable_index : Number
// }
const props = defineProps({
    color_space: {
        type: Object,
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
let color = defineModel("color");

// Only used for the Handle's background color.
const input_color_as_hex = computed(() => {
    return props.color_space.conversions.toRGBHex(color.value);
});

// Some constants.
const slider_width_px = 500;
const slider_height_px = 15;
const handle_width_px = 24;

const initial_handle_pos_px = slider_width_px / 2;

let slider_transform_style = ref({
    transform: handleTransformStyle(initial_handle_pos_px),
});

let pressed = false;

function hexToBytes(hex) {
    let bytes = [];
    for (let c = 0; c < hex.length; c += 2)
        bytes.push(parseInt(hex.substr(c, 2), 16));
    return bytes;
}
function truncToTwoDecimalPlaces(n) {
    return Math.floor(n * 100) / 100;
}
function handlePosXToVariable(x) {
    return truncToTwoDecimalPlaces((x / slider_width_px) * props.max_value);
}
function colorToHandlePosX(color_in) {
    return (color_in[props.variable_index] * slider_width_px) / props.max_value;
}

function handlePosXToHexColor(x) {
    // Create the color in the user's color space.
    // It uses the color provided in props.in_color as the base,
    // then modifies the variable at props.variable_index with the
    // value the slider represents.
    let color_space_color = color.value.with(
        props.variable_index,
        handlePosXToVariable(x),
    );

    // Convert the color from the user's color space to sRGB as hex.
    let hex_color = props.color_space.conversions.toRGBHex(color_space_color);

    // Assert that it returns only 6 characters, otherwise it isn't a 3-value hex code!
    if (hex_color.length != 6) {
        console.log(
            `uh oh, the hex color ${hex_color} has more than 6 characters!`,
        );
    }

    return hex_color;
}

function handleTransformStyle(x) {
    return `translate(${x - slider_width_px - handle_width_px / 2}px,0px)`;
}

function moveHandle(x, old_x) {
    if (0 > x) {
        // if the handle is on the left of the slider...
        x = 0;
    } else if (slider_width_px < x) {
        // if it is on the right of the slider...
        x = slider_width_px;
    }

    // Actually move the slider.
    slider_transform_style.value = { transform: handleTransformStyle(x) };
    return x;
}

function render() {
    // render the hue color slider
    if (canvas_el.value.getContext) {
        const ctx = canvas_el.value.getContext("2d");

        let img_arr = new Uint8ClampedArray(
            slider_width_px * slider_height_px * 4,
        );
        for (let i = 0; i < slider_width_px; i++) {
            let hex_color = handlePosXToHexColor(i);
            let new_color = hexToBytes(hex_color);
            new_color.push(255);
            for (let j = 0; j < slider_height_px; j++) {
                img_arr.set(new_color, (j * slider_width_px + i) * 4);
            }
        }

        let img_data = new ImageData(
            img_arr,
            slider_width_px,
            slider_height_px,
            {
                colorSpace: "srgb",
            },
        );
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
    let left_bound_absolute = bound.left - handle_width_px / 2;

    slider_el.value.addEventListener("mousedown", (e) => {
        pressed = true;

        // If the window is resized, the left bound of the slider will change; update it.
        left_bound_absolute =
            slider_el.value.getBoundingClientRect().left - handle_width_px / 2;

        let new_x = e.x - left_bound_absolute - handle_width_px / 2;
        slider_transform_style.value = {
            transform: handleTransformStyle(new_x),
        };

        // I am writing it like this because if I write:
        // `color = ...` it would overwrite the ref, not the ref's value.
        // And if I write `color[props.variable_index] = ...` then I am not overwriting
        // the array object which means the 'watch(...)' aren't going to pick
        // up that there wa a change in color.
        color.value[props.variable_index] = handlePosXToVariable(new_x);
    });
    window.addEventListener("mousemove", (e) => {
        if (!pressed) return;
        e.preventDefault();

        // Move the handle within the bounds of the slider.
        // slider space [0-slider_width]
        let new_x = e.x - left_bound_absolute - handle_width_px / 2;
        new_x = moveHandle(new_x);

        // Send the new color variable to the parent component.
        color.value[props.variable_index] = handlePosXToVariable(new_x);
    });

    window.addEventListener("mouseup", () => {
        pressed = false;
    });

    // Inital render and color output. Happens once.
    render();
    color.value[props.variable_index] = handlePosXToVariable(
        initial_handle_pos_px,
    );
});

// Whenever the parent's color changes, re-render the slider.
// Since we are watching a prop ( a model ) that has an array, we need to use `.value`.
//  Otherwise, we would only say `color` if it was just a number. Something about deep reactivity I think.
watch(color.value, (new_val, old_val) => {
    render();
    moveHandle(colorToHandlePosX(new_val));
});
</script>

<template>
    <div ref="slider-ref" id="slider">
        <canvas
            ref="color-slider-ref"
            id="color-slider"
            :width="slider_width_px"
            :height="slider_height_px"
        >
            There should be a color slide here
        </canvas>
        <svg id="handle" :style="slider_transform_style">
            <circle
                cx="12"
                cy="12"
                r="11"
                :fill="'#' + input_color_as_hex"
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
    margin: 0.25rem;
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
