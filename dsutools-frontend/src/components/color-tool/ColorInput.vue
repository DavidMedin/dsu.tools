<script setup>
import { ref, computed, inject } from "vue";
import ColorSlider from "./ColorSlider.vue";

// Controlled by the sliders.
// Takes in an index into the array
const props = defineProps({
    color_space_index: {
        type: Number,
        required: true,
    },
});
const color = defineModel({ required: true });

const color_spaces = inject("ColorSpaces");

let toRGBHex = color_spaces[props.color_space_index].conversions.toRGBHex;
let fromRGBHex = color_spaces[props.color_space_index].conversions.fromRGBHex;
// Computed from 'color' whenever it changes.
// sRGB can be though of as simply RGB. I don't know much about color theory :(
const srgb_color_hex = computed(() => {
    return toRGBHex(color.value);
});
</script>

<template>
    <div class="color-input">
        <div
            id="color-view"
            :style="{
                backgroundColor: '#' + srgb_color_hex,
            }"
        ></div>

        <input type="text" />
        <div class="slider-div">
            <input
                type="number"
                step="0.01"
                class="text-input"
                v-model="color[0]"
            />
            <ColorSlider
                :color_space="toRGBHex"
                :in_color="color"
                :max_value="360"
                :variable_index="0"
                v-model="color[0]"
            />
        </div>

        <div class="slider-div">
            <input
                type="number"
                step="0.01"
                class="text-input"
                v-model="color[1]"
            />
            <ColorSlider
                :color_space="toRGBHex"
                :in_color="color"
                :max_value="120"
                :variable_index="1"
                v-model="color[1]"
            />
        </div>

        <div class="slider-div">
            <input
                type="number"
                step="0.01"
                class="text-input"
                v-model="color[2]"
            />
            <ColorSlider
                :color_space="toRGBHex"
                :in_color="color"
                :max_value="100"
                :variable_index="2"
                v-model="color[2]"
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
