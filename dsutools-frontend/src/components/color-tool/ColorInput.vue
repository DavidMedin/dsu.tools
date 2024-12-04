<script setup>
import { computed, inject } from "vue";
import { serialize, get } from "colorjs.io/fn";
import ColorSlider from "./ColorSlider.vue";
import {
    truncToTwoDecimalPlaces,
    fmt_convert,
    to_hex,
} from "../../colorUtils.js";

// Controlled by the sliders.
// Takes in an index into the array
const props = defineProps({
    color_space_index: {
        type: Number,
        required: true,
    },
});
const color = defineModel("color");

const color_spaces = inject("ColorSpaces");
let space = computed(() => color_spaces[props.color_space_index]);
// color_spaces;
// Computed from 'color' whenever it changes.
// sRGB can be though of as simply RGB. I don't know much about color theory :(
const srgb_color_hex = computed(() => {
    // return space.value.conversions.toRGBHex(color.value);
    // return serialize(color.value, { format: "hex" });
    return to_hex(color.value);
});

const space_variables = computed(() =>
    Object.keys(space.value.coords).map((k) => space.value.coords[k]),
);

const color_getters = {};
for (const [key, value] of Object.entries(space.value.coords)) {
    color_getters[key] = computed({
        get: () => get(color.value, [space.value, key]),
        set: (new_value) => set(color.value, [space.value, key], new_value),
    });
}
</script>

<template>
    <div class="color-input">
        <div
            id="color-view"
            :style="{
                backgroundColor: srgb_color_hex,
            }"
        ></div>

        <input type="text" />
        <div class="slider-div" v-for="(coord, index) in space.coords">
            <label>{{ coord.name }}</label>
            <ColorSlider
                :color_space="space"
                v-model:color="color"
                :coord_name="index"
            />
        </div>
    </div>
</template>

<style scoped>
.color-input {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
.slider-div {
    display: flex;
    flex-direction: row;
    align-items: center;
}

.copyable-text {
    height: 2.5em;
    color: white;
    background-color: var(--color-background-mute);
    border-color: var(--color-primary);
    border-radius: 0.4rem;
    padding: 0.5em;
    text-align: center;
    display: flex;
    align-items: center;
}

.copy-button {
    width: 1em;
    height: 1em;
    padding: 0;
    margin: 0;
    background-color: var(--color-primary);
    border-radius: 0.2rem;
}

.text-input {
    width: 4em;
}

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
</style>
