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
        <p id="title">{{ tool_name || "MISSING TOOL NAME" }}</p>

        <slot></slot>
    </a>
</template>

<style scoped>
/* Don't style the anchor. */
a {
    all: revert;
    text-decoration: none;
    box-sizing: border-box;
}

/* Style text to look normal. */
* {
    color: var(--color-text);
}

.tool {
    background-color: var(--color-background-soft);
    border-radius: 0.5em;
    padding: 0.5em;
    margin: 0.5em;
    min-height: 5em;
    width: 100%;
    height: 100%;
    border-color: hsla(160, 100%, 37%, 1);
    border-width: 0.1em;
    border-style: solid;
}

.tool:hover {
    background-color: var(--color-background-mute);
}

#title {
    text-align: center;
}
</style>
