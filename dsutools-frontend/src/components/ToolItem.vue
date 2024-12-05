<script setup>
defineProps({
    tool_name: {
        type: String,
        required: true,
    },
});
</script>

<template>
    <div class="round-box">
        <p id="title">{{ tool_name || "MISSING TOOL NAME" }}</p>

        <slot />
    </div>
</template>

<style scoped>
.round-box {
    width: 100%;
    height: 100%;
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;    
    text-decoration: none;
    font-size: 1.5rem;
}

#title {
    text-align: center;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: white;
}

</style>
<!-- .round-box {
    width: 100%;
    height: 100%;
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;
}

#title {
    text-align: center;
    text-transform: uppercase;
    font-size: 1.5rem;
    color: white;
}-->