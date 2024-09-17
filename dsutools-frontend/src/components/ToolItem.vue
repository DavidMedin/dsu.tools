<script setup>
defineProps({
    tool_name: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <a class="box-link" href="/irina.html">
        <div class="tool">
            <p>{{ tool_name || "MISSING TOOL NAME" }}</p>
        </div>
    </a>
</template>

<style scoped>
.tool {
    background-color: var(--color-background-soft);
    border-radius: 0.5em;
    padding: 0.5em;
    margin: 0.5em;
    min-height: 5em;
}

.tool:hover {
    background-color: var(--color-background-mute);
}

.box-link:hover {
    background-color: revert;
}
</style>
