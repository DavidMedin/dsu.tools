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
}

#title {
    text-align: center;
}
</style>
