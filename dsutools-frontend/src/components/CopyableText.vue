<script setup>
import { IconClipboard } from "@tabler/icons-vue";
import ToolTip from "./ToolTip.vue";
const props = defineProps({
    label: {
        type: String,
    },
    text: {
        required: true,
    },
});

function copy() {
    // TODO: Check permissions before writing to the clipboard.

    // Copy props.text to the clipboard.
    navigator.clipboard.writeText(props.text).then(
        () => {
            /* clipboard successfully set */
        },
        () => {
            /* clipboard write failed */
            console.log("Failed to copy text...");
        },
    );
}
</script>

<template>
    <div class="outer tooltip-haver" @click="copy">
        <ToolTip />
        <IconClipboard class="icon" />
        <div class="content">
            <p unselectable="on" onselectstart="return false;">
                {{ (props.label || "") + text }}
            </p>
        </div>
    </div>
</template>

<style scoped>
.outer {
    display: flex;
    flex-direction: row;
    align-items: center;
    background-color: var(--color-surface-lvl-1);
    border-radius: 0.25rem;
}
.outer:hover {
    background-color: var(--color-surface-lvl-2);
    transform: scale(1.05);
}

.icon {
    /* width: 1rem; */
    /* height: 1rem; */
    /* margin: 0.2rem; */
    /* padding-left: 0.2rem; */
    height: 1em;
    width: 1em;
    margin-left: 0.25em;
    margin-right: 0.25em;
    color: var(--color-primary);
}
/* .copy-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 0.25rem;
    background-color: var(--color-surface-lvl-2);
} */
.content {
    /* Flex Child options (.content is a flex child to .outer) */
    flex-grow: 1;

    /* Flex Parent options (.content is a flex parent to the p element.) */
    display: flex;
    flex-direction: row;
    justify-content: center;
    /* width: 100%; */

    margin-right: 0.5rem;
    /* margin-left: 0.5rem; */
}
.content p {
    -moz-user-select: none;
    -webkit-user-select: none;
    -ms-user-select: none;
    user-select: none;
    -o-user-select: none;
}
</style>
