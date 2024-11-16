<script setup>
defineProps({
    tool_name: {
        type: String,
        required: true,
    },
    tool_link: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <a id="outer-link" v-bind:href="tool_link">
        <div class="round-box">
            <p id="title">{{ tool_name || "MISSING TOOL NAME" }}</p>
            <!-- <p> lorem50</p> -->
            <slot />
        </div>
    </a>
</template>

<style scoped>
/* Don't style the anchor. */
#outer-link {
    all: revert;
    text-decoration: none;
    width: 100%;
    height: 100%;
    font-size: 1.5rem;
}

/* Style text to look normal. */
* {
    color: var(--color-text);
}

.round-box {
    width: 100%;
    height: 100%;
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;
}

.round-box:hover {
    background-color: var(--color-background-mute);
    border-color: var(--color-tertiary);
}

#title {
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: white;
}
</style>
