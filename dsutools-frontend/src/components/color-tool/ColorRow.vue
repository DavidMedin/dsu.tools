<script setup>
import { ref } from "vue"
import ColorSpaceSelector from "./ColorSpaceSelector.vue"
import ColorOutput from "./ColorOutput.vue"
import ColorInput from "./ColorInput.vue"

// TODO: panic if conversions is not set.

// For now, this is an HCT color.
let color = ref([0,0,0])

// index into the global 'ColorSpaces' object. Get it with inject("ColorSpaces")
let color_space_index = ref(0)
</script>

<template>
<div class="row">
    <ColorSpaceSelector :selected_color_space="color_space_index"/>
    <ColorInput :color_space_index="color_space_index" v-model="color"/>
    <ColorOutput :color="color" :color_space="color_space"/>

</div>
</template>

<style scoped>
.row {
    display: flex;
    flex-direction: row;
}
</style>
