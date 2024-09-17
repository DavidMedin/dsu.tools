<script setup>
defineProps({
    tool_name: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <a class="box-link tool" href="/irina.html">
        <p>{{ tool_name || "MISSING TOOL NAME" }}</p>

        <slot></slot>
    </a>
</template>

<style scoped>
.tool {
    background-color: var(--color-background-soft);
    border-radius: 0.5em;
    padding: 0.5em;
    margin: 0.5em;
    min-height: 5em;
    width: 100%;
    height: 100%;
}

.tool:hover {
    background-color: var(--color-background-mute);
}

.box-link:hover {
    background-color: revert;
}
</style>
