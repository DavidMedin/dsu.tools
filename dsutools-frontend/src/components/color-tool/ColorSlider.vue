<script setup>
import { onMounted, useTemplateRef, watch, computed, ref } from "vue";
import {
    serialize,
    to as convert,
    set,
    get,
    toGamut,
    sRGB,
    sRGB_Linear,
} from "colorjs.io/fn";
import {
    truncToTwoDecimalPlaces,
    fmt_convert,
    to_hex,
} from "../../colorUtils.js";
// Input/Output Model for the Color Slider:
// {
// [parameter] colorSpace : ColorJS.io colorspace object
// [input] (model) color : colorJS.io color object
// [input] coord_name : string
// }
const props = defineProps({
    color_space: {
        type: Object,
        required: true,
    },
    coord_name: {
        type: String,
        required: true,
    },
});
let color = defineModel("color");
const color_var_range = computed(() => {
    let coord = props.color_space.coords[props.coord_name];
    if (coord.range == null) {
        return coord.refRange;
    } else {
        return coord.range;
    }
});
// Only used for the Handle's background color.
const input_color_as_hex = computed(() => {
    return to_hex(color.value);
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

function handlePosXToVariable(x) {
    return truncToTwoDecimalPlaces(
        (x / slider_width_px) *
            (color_var_range.value[1] - color_var_range.value[0]) +
            color_var_range.value[0],
    );
}
function colorToHandlePosX(color_in) {
    return (
        (get(color_in, [props.color_space, props.coord_name]) *
            slider_width_px) /
        color_var_range.value[1]
    );
}

function handlePosXToBytes(x) {
    // Create the color in the user's color space.
    // It uses the color provided in props.in_color as the base,
    // then modifies the variable at props.variable_index with the
    // value the slider represents.

    // Create a copy of the color so we don't modify the original color.
    let tmp_color = { ...props.color };
    set(
        tmp_color,
        [props.color_space, props.coord_name],
        handlePosXToVariable(x),
    );

    // Convert the color from the user's color space to sRGB as hex.
    let rgb_color = convert(tmp_color, sRGB_Linear);
    let bytes = [
        rgb_color.coords[0] * 255,
        rgb_color.coords[1] * 255,
        rgb_color.coords[2] * 255,
        255,
    ];

    return bytes;
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
    if (canvas_el.value == null) {
        return;
    }
    if (canvas_el.value.getContext) {
        const ctx = canvas_el.value.getContext("2d");

        let img_arr = new Uint8ClampedArray(
            slider_width_px * slider_height_px * 4,
        );
        for (let i = 0; i < slider_width_px; i++) {
            let new_color = handlePosXToBytes(i);
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

        set(
            color.value,
            [props.color_space, props.coord_name],
            handlePosXToVariable(new_x),
        );
    });
    window.addEventListener("mousemove", (e) => {
        if (!pressed) return;
        e.preventDefault();

        // Move the handle within the bounds of the slider.
        // slider space [0-slider_width]
        let new_x = e.x - left_bound_absolute - handle_width_px / 2;
        new_x = moveHandle(new_x);

        // Send the new color variable to the parent component.
        set(
            color.value,
            [props.color_space, props.coord_name],
            handlePosXToVariable(new_x),
        );
    });

    window.addEventListener("mouseup", () => {
        pressed = false;
    });

    // Inital render and color output. Happens once.
    render();
    set(
        color.value,
        [props.color_space, props.coord_name],
        handlePosXToVariable(initial_handle_pos_px),
    );
});

// Whenever the parent's color changes, re-render the slider.
watch(color.value, (new_val, old_val) => {
    render();
    moveHandle(colorToHandlePosX(new_val));
});

// Watch for the case where the color doesn't change when the color space changes ( like with the color #000000).
watch(
    () => props.color_space,
    (new_val, old_val) => {
        render();
        moveHandle(colorToHandlePosX(color.value));
    },
);
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
            <circle cx="12" cy="12" r="11" :fill="input_color_as_hex"></circle>
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
