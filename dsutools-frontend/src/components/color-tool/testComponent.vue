<script setup>
import { defineModel, watch } from "vue";
const number = defineModel("number");
const things = defineModel("things");

const number_type = typeof number.value;
console.log("hello!");
let random_number = Math.floor(Math.random() * 100);
let sometimes = 0;
function indirection() {
    if (sometimes == 1) {
        // things.value = things.value.with(0, random_number);
        things.value[0] = random_number;
    }
    sometimes += 1;
    sometimes %= 2;
}
watch(number, () => {
    console.log("randomizing");
    random_number = Math.floor(Math.random() * 100);
    indirection();
});

let counter = 0;
watch(things.value, () => {
    counter += 1;
});
</script>

<template>
    <p>test component</p>
    <p>type : {{ number_type }}</p>
    <p>things : {{ things }}</p>
    <p>counter : {{ counter }}</p>
    <input type="number" v-model="things[0]" />
    <p>
        a number ---> <span>{{ number }}</span> <---
    </p>
    <p>A random number ---> {{ random_number }} <---></p>
</template>

<style scoped></style>
