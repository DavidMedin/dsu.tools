<script setup>
import {ref, computed} from 'vue'
import Page from "./components/Page.vue";
import Vitamin from "./components/Vitamin.vue";

const symptom = ref("Any Symptom");


//stores all vitamin symptoms
const vitamins = ref([
    {vitamin: "A", High: ["happy", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminA-Consumer/#:~:text=professional%20fact%20sheet.-,What%20are%20vitamin%20A%20and%20carotenoids%20and%20what%20do%20they,and%20other%20organs%20work%20properly"},
    {vitamin: "C", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminC-HealthProfessional/"},
    {vitamin: "D", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminD-Consumer/"},
    {vitamin: "E", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminE-Consumer/"},
    {vitamin: "K", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminK-Consumer/"},
    {vitamin: "B-Thiamine", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/Thiamin-Consumer/"},
    {vitamin: "B-Riboflavin", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/Riboflavin-Consumer/"},
    {vitamin: "B-Niacin", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/Niacin-Consumer/"},
    {vitamin: "B-Pantothenic Acid", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/PantothenicAcid-Consumer/"},
    {vitamin: "B-Biotin", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/Biotin-Consumer/"},
    {vitamin: "B6", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminB6-Consumer/"},
    {vitamin: "B12", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/VitaminB12-Consumer/"},
    {vitamin: "B-Folate", High: ["tired", "anxious"], Low: ["tired", "anxious"], Link: "https://ods.od.nih.gov/factsheets/Folate-Consumer/"}
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
    return symptom.value === "Any Symptom"
    ? vitamins.value
    : vitamins.value.filter((v => checkStrings(v.High, symptom.value))) 
});



//Filters vitmain list to just the vitmains that contain the selected low symptom
const filteredVitaminsLow = computed(() => {
    return symptom.value === "Any Symptom"
    ? vitamins.value
    : vitamins.value.filter((v => checkStrings(v.Low, symptom.value))) 
});

function checkStrings(strings, string) {
    if (!Array.isArray(strings)) return false; // Safeguard against invalid input
    return strings.some((s) => s === string);
}

</script>

<template>
    <Page>
        <div id="page-contents">
            <div id="tool-description">
                <h2>Vitamin Calculator</h2>
                <h3>Choose a symptom:</h3>
                <div id="dropdown">
                    <select v-model="symptom">
                        <option value="Any Symptom">Any Symptom</option>
                        <option v-for="s in symptoms" :key="s" :value="s">
                            {{ s }}
                        </option>
                    </select>
                </div>
                <p id="description">
                    Select a symptom above to show the vitamin surpluses and
                    deficiencies that would cause the selcted symptom.
                </p>
                <p id="disclaimer">
                    This tool is not to be used for medical advice. This tools 
                    intended use is for determining potential causes of 
                    symptoms. If the user has concerns about deficiencies/
                    surpluses in vitamins they should contact a health 
                    professional. DSU.tools nor it's creators are not liable 
                    for any improper use of this information.
                </p>
            </div>  <!--End of left-side.-->
            <div style="display:flex; gap: 2.5%; width:80%" id="right-side">
                <div class="round-box" style="width:50%" id="low">
                    <div class="header-box">
                        <h2 class="list-title">list of low vitamins</h2>
                    </div>
                    <div class="list">
                        <div v-for="v in filteredVitaminsLow" :key="v" style="display: flex; width: 47%;">
                            <Vitamin :vitamin_name = v.vitamin :low_symptoms = v.Low :high_symptoms = v.High
                            :vitamin_link = v.Link></Vitamin>
                        </div>
                    </div>
                </div>
                <div class="round-box" style="width:50%" id="high">
                    <div class="header-box">
                        <h2 class="list-title">list of high vitamins</h2>
                    </div>
                    <div class="list">
                        <div v-for="v in filteredVitaminsHigh" :key="v" style="display: flex; width: 47%;">
                            <Vitamin :vitamin_name = v.vitamin :low_symptoms = v.Low :high_symptoms = v.High
                            :vitamin_link = v.Link></Vitamin>
                        </div>
                    </div>
                </div>
            </div>  <!--End of right-side.-->
        </div>  <!--End of page-content.-->  
    </Page>
</template>

<style scoped>

#page-contents{
    display: flex; 
    gap: 2.5%; 
    width: 100%
}

#tool-description{
    margin: 1rem;
    margin-top: 0;
    width: 20%;
}

#description{
    margin-top: 1rem;
    color: var(--color-primary);
}

#disclaimer{
    color: red;
    font-size:xx-small;
    margin-top: 9.8rem;
}

#right-side{
    margin: 2rem;
}

h2{
    text-align: center;
}

.round-box{
    height: 29rem;
    padding: 0;
}

.header-box{
    background-color: var(--color-secondary);
    border-radius: 1em;
    width: 70%;
    height: 10%;
    margin-left: 15%;
    margin-right: 15%;
}

.list-title{
    color: var(--vt-c-black-soft);
}

.list{
    max-height: 82%;
    overflow-y: auto;
    margin: .5rem;
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
}

.list::-webkit-scrollbar {
    background-color: var(--vt-c-black-soft);
    border-radius: .5rem;
}

.list::-webkit-scrollbar-thumb {
    background: var(--color-secondary);
    border-radius: 1rem;
}

</style>