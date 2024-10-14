<script setup>
import {ref, computed} from 'vue'
import Page from "./components/Page.vue";

//dictonary for storing information;
const symptom = ref("tired");
const symptoms = ref(["tired", "anxious", ])

const vitamins = ref([
    {vitamin: "A", High: ["happy", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "C", High: [symptoms.value[0], symptoms.value[1]], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "D", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "E", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "K", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B-Thiamine", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B-Riboflavin", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B-Niacin", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B-Pantothenic Acid", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B-Biotin", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B6", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B12", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]},
    {vitamin: "B-Folate", High: ["tired", "anxious"], Low: [symptoms.value[1], symptoms.value[0]]}
])


const filteredVitaminsHigh = computed(() => {
    return symptom.value === ""
    ? null
    : vitamins.value.filter((v => checkStrings(v.High, symptom.value))) 
})

const filteredVitaminsLow = computed(() => {
    return symptom.value === ""
    ? null
    : vitamins.value.filter((v => checkStrings(v.Low, symptom.value))) 
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
                    2 list of matching symptoms for either too much or too little of a vitamin.</p>
                    <label for="symptoms">Choose a vitamin:</label>
                    <select id="symptoms">
                        <option value="null"></option>
                        <option v-for="v in vitamins" :key="v.vitamin">
                            {{ v.vitamin }}
                        </option>
                    </select>
            </div>
            <div id="low">
                <h3>list of low vitamins</h3>
                <ul>
                    <li v-for="v in filteredVitaminsLow" :key="v.vitamin">
                        <span>{{ v.vitamin }}</span>
                    </li>
                </ul>
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



/*

List of Primary 13 vitamins: https://nia.nih.gov/health/vitamins-and-supplements/vitamins-and-minerals-older-adults

A: https://ods.od.nih.gov/factsheets/VitaminA-Consumer/#:~:text=professional%20fact%20sheet.-,What%20are%20vitamin%20A%20and%20carotenoids%20and%20what%20do%20they,and%20other%20organs%20work%20properly.
C: https://ods.od.nih.gov/factsheets/VitaminC-HealthProfessional/
D: https://ods.od.nih.gov/factsheets/VitaminD-Consumer/
E: https://ods.od.nih.gov/factsheets/VitaminE-Consumer/
K: https://ods.od.nih.gov/factsheets/VitaminK-Consumer/
B-Thiamine: https://ods.od.nih.gov/factsheets/Thiamin-Consumer/
B-Riboflavin: https://ods.od.nih.gov/factsheets/Riboflavin-Consumer/
B-Niacin: https://ods.od.nih.gov/factsheets/Niacin-Consumer/
B-Pantothenic Acid: https://ods.od.nih.gov/factsheets/PantothenicAcid-Consumer/
B-Biotin: https://ods.od.nih.gov/factsheets/Biotin-Consumer/
B6: https://ods.od.nih.gov/factsheets/VitaminB6-Consumer/
B12: https://ods.od.nih.gov/factsheets/VitaminB12-Consumer/
B-Folate: https://ods.od.nih.gov/factsheets/Folate-Consumer/

*/