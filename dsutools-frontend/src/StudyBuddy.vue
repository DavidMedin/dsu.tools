<script setup>
import Flashcard from "./components/Flashcard.vue";
</script>

<template>
    <main>
        <Flashcard></Flashcard>
    </main>
</template>

<style scoped>
    * {
        font-family: 'Courier New', Courier, monospace;
        color: black;
    }

    main {
        display: flex;
        align-items: center;
        justify-content: center;
        flex-wrap: wrap;
        position: absolute;
    }
</style>