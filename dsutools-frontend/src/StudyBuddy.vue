<script setup>
import Flashcard from "./components/Flashcard.vue";
import Page from "./components/Page.vue";
</script>

<template>
    <Page direction="row" justifyContent="space-between" alignTimes="auto">
        <div class="sidebar">
            <button class="sticky-button">Create new flashcard set</button>
            <h3 id="saved-sets-header" style="color: var(--color-primary)">SAVED SETS</h3>
            <div id="saved-sets">
            </div>
        </div>
        <div class="flashcards-main">
        </div>
    </Page>
</template>

<style scoped>
.sidebar {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    width: 20%;
    gap: 1rem;
}

#saved-sets-header {
    color: var(--color-primary);
    font-weight: bold;
    letter-spacing: 0.4rem;
}

.sticky-button {
    position: sticky;
    top: 0;
}

.flashcards-main {
    width: 80%;
    height: 100%;
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;
    border-radius: 1em;
    padding: 1em;
    min-height: 5em;
    font-size: 1.5rem;
}
</style>