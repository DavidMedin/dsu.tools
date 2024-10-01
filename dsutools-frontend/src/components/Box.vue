<script setup>
defineProps({
    tool_name: {
        type: String,
        Required: true
    },
})
</script>

<template>
    <div class="box">
        <slot>{{ tool_name }}</slot>
    </div>
</template>

<style scoped>
.box {
        display: flex;
        border-radius: 5rem;
        justify-content: center;
        align-items: center;
        margin: 0.5rem 1rem 0.5rem 1rem;
        font-size: 2rem;
        color: var(--white);
    }
</style>