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
    grid-template-columns: 10vw 10vw;
    grid-auto-flow: dense;
    gap: 0.5em; /* vertical and horizontal */
    place-items: center; /* vertical and horizontal */
    place-content: center; /* place the entire grid. (h and v) */
    grid-auto-columns: 10em;
}
</style>
