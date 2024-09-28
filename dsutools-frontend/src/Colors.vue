<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import Page from "./components/Page.vue";
import ColorRow from "./components/color-tool/ColorRow.vue"

// Component Hierarchy
// -------------------
// ColorRow
// |>ColorInput
//   |>ColorSlider
// |>ColorOutput
</script>

<template>
    <Page>
        <p id="pregrid-text">Color Calculator</p>
        <div id="center">
            <ColorRow/>
        </div>
    </Page>
</template>

<style scoped>

#center {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>
