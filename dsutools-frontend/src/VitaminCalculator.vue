<script setup>
import {ref, computed} from 'vue'
import Page from "./components/Page.vue";

//dictonary for storing information;
const symptom = ref("happy");

const vitamins = ref([
    {vitamin: "d", High: ["happy", "anxious"]},
    {vitamin: "f", High: ["tired", "anxious"]}
])


const filteredVitaminsHigh = computed(() => {
    return symptom.value === ""
    ? null
    : vitamins.value.filter((v => checkStrings(v.High, symptom.value))) 
    
})

function checkStrings(strings, string){
    return strings.some(s => s == string);
}

</script>

<template>
    <Page >
        <div>
            <div id="dropdown">
                <p>dropdown menu to select symptom. On click it shows 
                    2 list of matching symptoms for either too much vitamins or too little.</p>
                    <label>Choose a vitamin:</label>
                    <select>
                        <option value="null"></option>
                        <option v-for="v in vitamins" :key="v.vitamin">
                            {{ v.vitamin }}
                        </option>
                    </select>
            </div>
            <div id="low">
                <h3>list of low vitamins</h3>
                
            </div>
            <div id="high">
                <h3>list of high vitamins</h3>
                <ul>
                    <li v-for="v in filteredVitaminsHigh" :key="v.vitamin">
                        <span>{{ v.vitamin }}</span>
                    </li>
                </ul>
            </div>
        </div>
    </Page>
</template>

<style scoped>
</style>