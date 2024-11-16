<script setup>
import {ref, computed} from 'vue'
import Page from "./components/Page.vue";

const symptom = ref(null);


//stores all vitamin symptoms
const vitamins = ref([
    {vitamin: "A", High: ["happy", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "C", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "D", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "E", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "K", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B-Thiamine", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B-Riboflavin", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B-Niacin", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B-Pantothenic Acid", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B-Biotin", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B6", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B12", High: ["tired", "anxious"], Low: ["tired", "anxious"]},
    {vitamin: "B-Folate", High: ["tired", "anxious"], Low: ["tired", "anxious"]}
])



// Dynamically derive symptoms from vitamins
const symptoms = computed(() => {
    const uniqueSymptoms = new Set();
    vitamins.value.forEach((v) => {
        v.High.forEach((symptom) => uniqueSymptoms.add(symptom));
        v.Low.forEach((symptom) => uniqueSymptoms.add(symptom));
    });
    return Array.from(uniqueSymptoms);
});



//Filters vitmain list to just the vitmains that contain the selected high symptom
const filteredVitaminsHigh = computed(() => {
    return symptom.value === null
    ? vitamins.value
    : vitamins.value.filter((v => checkStrings(v.High, symptom.value))) 
});



//Filters vitmain list to just the vitmains that contain the selected low symptom
const filteredVitaminsLow = computed(() => {
    return symptom.value === null
    ? vitamins.value
    : vitamins.value.filter((v => checkStrings(v.Low, symptom.value))) 
});

function checkStrings(strings, string) {
    if (!Array.isArray(strings)) return false; // Safeguard against invalid input
    return strings.some((s) => s === string);
}

</script>

<template>
    <Page >
        <div>
            <div id="dropdown">
                <p>dropdown menu to select symptom. On click it shows
                    2 list of matching symptoms for either too much or too little of a vitamin.</p>
                    <label for="symptoms">Choose a Symptom:</label>
                    <select v-model="symptom">
                        <option :value="null"></option>
                        <option v-for="s in symptoms" :key="s" :value="s">
                            {{ s }}
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