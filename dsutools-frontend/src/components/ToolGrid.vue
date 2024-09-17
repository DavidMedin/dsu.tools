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
    display: flex;
    flex-direction: row;
    justify-content: center;
}
</style>
