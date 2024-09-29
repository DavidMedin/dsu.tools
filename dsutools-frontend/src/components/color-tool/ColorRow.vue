<script setup>
import { ref } from "vue"
import ColorOutput from "./ColorOutput.vue"
import ColorInput from "./ColorInput.vue"

// TODO: panic if conversions is not set.

// For now, this is an HCT color.
let color = ref([0,0,0])

let color_space = "hct"
</script>

<template>
<div class="row">

    <ColorInput :color_space="color_space" v-model="color"/>
    <ColorOutput :color="color" :color_space="color_space"/>

</div>
</template>

<style scoped>
.row {
    display: flex;
    flex-direction: row;
}
</style>
