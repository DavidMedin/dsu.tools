<script setup>
defineProps({
    vitamin_name: {
        type: String,
        required: true,
    },
    vitamin_link: {
        type: String,
        required: true,
    },
    low_symptoms: {
        type: Array,
        required: true,
    },
    high_symptoms:{
        type: Array,
        required: true,
    },
});
</script>

<template>
    <a id="outer-link" v-bind:href="vitamin_link || 'vitamincalculator.html'" style="display: flex; width: 100%;">
        <div class="round-box" style="display: flex; width: 100%; flex-wrap: wrap; align-content: flex-start;">
            <div style="width: 100% ; height: fit-content;">
                <p id="title">{{ vitamin_name || "MISSING TOOL NAME" }}</p>
            </div>
            <div id="list-container">
                <div class="small-list">
                    <h6>Low</h6>
                    <ul style="padding: 0; padding-left: 1rem;">
                        <li v-for="symptom in low_symptoms" :key="symptom">
                            {{ symptom }}
                        </li>
                    </ul>
                </div>
                <div class="small-list">
                    <h6>High</h6>
                    <ul style="padding: 0; padding-left: 1rem;">
                        <li v-for="symptom in high_symptoms" :key="symptom">
                            {{ symptom }}
                        </li>
                    </ul>
                </div>
            </div>
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
    font-size: 1.5rem;
}

/* Style text to look normal. */
* {
    color: var(--color-text);
}

.round-box {
    width: 100%;
    height: 100%;
    background-color: var(--vt-c-black);
    margin: .25rem;
    padding-top: 0;
}

.round-box:hover {
    background-color: var(--color-background-mute);
}

#title {
    font-size: medium;
    font-weight: bold;
    text-align: center;
    text-transform: uppercase;
    color: white;
    width: 100%;
    height: fit-content;
}

#list-container{
    display: flex;
    width: 100%;
}

.small-list{
    width: 50%;
}

li{
    font-size: xx-small;
}
</style>