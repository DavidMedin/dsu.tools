<script setup>
const props = defineProps({
    options: {
        type: Array,
        required: true,
    },
});
const selected_index = defineModel({ required: true });

selected_index.value = 0;
</script>

<template>
    <div>
        <p>selected : {{ options[selected_index] }}</p>
    </div>
</template>

<style scoped></style>
