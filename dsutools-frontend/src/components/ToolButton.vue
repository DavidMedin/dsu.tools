<script setup>
defineProps({
    tool_name: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <a id="outer-link" href="/irina.html">
        <div class="round-box">
            <p id="title">{{ tool_name || "MISSING TOOL NAME" }}</p>

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
}

/* Style text to look normal. */
* {
    color: var(--color-text);
}

.round-box {
    width: 100%;
    height: 100%;
    border-color: rgb(0, 169, 224);
    border-width: 0.1em;
    border-style: solid;
}

.round-box:hover {
    background-color: var(--color-background-mute);
}

#title {
    text-align: center;
}
</style>
