<script setup></script>

<template>
    <div class="grid">
        <slot>
            <p>Fallback Slot Content</p>
        </slot>
    </div>
</template>

<style scoped>
.grid {
    display: grid;
    grid-template-columns: 20vw 20vw 20vw;
    grid-auto-flow: dense;

    /* vertical and horizontal */
    gap: 0.5em;

    /* place the entire grid. (h and v) */
    place-content: center;
    grid-auto-rows: 20em;

    margin: 1em;
    /* vertical and horizontal */
    place-items: center;
}
</style>
