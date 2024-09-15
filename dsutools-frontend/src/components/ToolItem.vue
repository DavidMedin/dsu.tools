<script setup>
defineProps({
    tool_name: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <div class="tool">
        <p>{{ tool_name || "MISSING TOOL NAME" }}</p>
    </div>
</template>

<style scoped>
.tool {
    background-color: var(--color-background-mute);
    border-radius: 0.5em;
    padding: 0.5em;
    margin: 0.5em;
    min-height: 5em;
}
</style>
