<script setup>
import { ref } from "vue";

const props = defineProps({
    isSetSelected: {
        type: Boolean
    },
    deckName: {
        type: String,
        required: true,
    },
    functionOnClick: {
        type: Function,
        required: true,
    },
});
</script>

<template>
    <button class="sticky-button" @click="props.functionOnClick">Create new flashcard</button>
    <div>Deckname is: {{ deckName }}</div>
</template>

<style scoped>
</style>
