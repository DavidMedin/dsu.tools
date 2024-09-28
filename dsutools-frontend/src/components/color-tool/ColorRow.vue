<script setup>
import ColorOutput from "./ColorOutput.vue"
import ColorInput from "./ColorInput.vue"
</script>

<template>
<div class="row">

    <ColorInput/>
    <ColorOutput/>

</div>
</template>

<style scoped>
.row {
    display: flex;
    flex-direction: row;
}
</style>
