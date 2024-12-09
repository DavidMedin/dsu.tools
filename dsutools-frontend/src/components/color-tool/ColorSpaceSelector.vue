<script setup>
import {ref,inject } from 'vue'
import DropDownSelect from '../DropDownSelect.vue';

let color_spaces = inject("ColorSpaces")

// Create an array of the names of the color spaces so that the DropDownSelect can consume it.
const options = color_spaces.map((space) => space.name)
let selected = defineModel("selected_color_space", {required:true})
</script>

<template>
<div class="space">
    <DropDownSelect :options="options" v-model="selected"/>
    <p>Dropdown selected {{color_spaces[selected].name}}!</p>
    <p>Dropdown selected {{color_spaces[selected].description || "empty desc"}}!</p>
</div>
</template>

<style scoped>
.space {
   height:100%
}
</style>
