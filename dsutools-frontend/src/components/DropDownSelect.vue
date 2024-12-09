<script setup>
const props = defineProps({
    options: {
        type: Array,
        required: true,
    },
});
const selected_index = defineModel({ required: true });

selected_index.value = 0;

function react_to_select_input(e) {
    selected_index.value = e.target.options.selectedIndex;
}
</script>

<template>
    <div>
        <select @input="react_to_select_input">
            <option v-for="item in options" :value="item">
                {{ item }}
            </option>
        </select>
    </div>
</template>

<style scoped></style>
