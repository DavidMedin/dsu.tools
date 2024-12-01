<script setup>
import { ref, watch, inject } from "vue";
import ColorSpaceSelector from "./ColorSpaceSelector.vue";
import ColorOutput from "./ColorOutput.vue";
import ColorInput from "./ColorInput.vue";

// For now, this is an HCT color.
//Be careful! Do NOT set the whole array, only the components individually!
let color = ref([0, 0, 0]);

// index into the global 'ColorSpaces' object. Get it with inject("ColorSpaces")
const color_spaces = inject("ColorSpaces");
let color_space_index = ref(0);

watch(color_space_index, (new_val, old_val) => {
    let old_hex = color_spaces[old_val].conversions.toRGBHex(color.value);
    let new_color = color_spaces[new_val].conversions.fromRGBHex(old_hex);
    color.value[0] = new_color[0];
    color.value[1] = new_color[1];
    color.value[2] = new_color[2];
});
</script>

<template>
    <div class="row">
        <ColorSpaceSelector v-model:selected_color_space="color_space_index" />
        <ColorInput
            :color_space_index="color_space_index"
            v-model:color="color"
        />
        <ColorOutput :color="color" :color_space_index="color_space_index" />
    </div>
</template>

<style scoped>
.row {
    display: flex;
    flex-direction: row;
}
</style>
