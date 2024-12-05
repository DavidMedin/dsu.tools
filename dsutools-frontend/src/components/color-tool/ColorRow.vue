<script setup>
import { ref, watch, inject } from "vue";
import ColorSpaceSelector from "./ColorSpaceSelector.vue";
import ColorOutput from "./ColorOutput.vue";
import ColorInput from "./ColorInput.vue";

// For now, this is an HCT color.
//Be careful! Do NOT set the whole array, only the components individually!
let color = ref({ space: "hct", coords: [0, 50, 50] });

// index into the global 'ColorSpaces' object. Get it with inject("ColorSpaces")
let color_space_index = ref(0);
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
