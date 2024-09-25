<!-- This file will be imported by colors.js by colors.html. -->

<script setup>
import Page from "./components/Page.vue";
import ColorSlider from "./components/ColorSlider.vue";
</script>

<template>
    <Page>
        <p id="pregrid-text">Color Calculator</p>
        <div id="center">
            <ColorSlider />
        </div>
    </Page>
</template>

<style scoped>
#pregrid-text {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    text-align: center;
}

#center {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}
</style>
