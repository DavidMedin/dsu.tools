<script setup>
import { onMounted, ref } from "vue";
import Flashcard from "./components/Flashcard.vue";
import Page from "./components/Page.vue";
import FlashcardSetOptions from "./components/FlashcardSetOptions.vue";

const flashcards = ref([]);
const savedSets = ref([]);
const selectedSet = ref(null);
const isFlashcardSetFormVisible = ref(false);
const isFlashcardFormVisible = ref(false);
const errorMessage = ref("");

function toggleVisibility() {
    isFlashcardSetFormVisible.value = !isFlashcardSetFormVisible.value;
}

function selectSet(set) {
    selectedSet.value = set;
    fetchDecksFlashcards(set.name);
    localStorage.setItem("currentDeck", set.name);
    selectSet.value = set;
}

// fetch the flashcards for a specific deck
function fetchDecksFlashcards(deckName) {
    if (localStorage.getItem("username") == null) {
        alert("Please log in to view your flashcard sets!");
        return;
    }
    fetch(`/flashcard-deck?username=${localStorage.getItem("username")}&flashcard_deck_name=${deckName}`, {
        method: "GET",
    })
        .then(function(response) {
            return response.json();
        })
        .then(function(data) {
            console.log(data);
            flashcards.value = data;
        })
        .catch(function(error) {
            console.error("Error: ", error);
        });
}

// fetch the saved flashcard decks
function fetchSavedFlashcardSets() {
    if (localStorage.getItem("username") == null) {
        alert("Please log in to view your flashcard sets!");
        return;
    }
    fetch(`/get-flashcard-decks?username=${localStorage.getItem("username")}`, {
        method: "GET",
        })
        .then(function(response) {
            return response.json();
        })
        .then(function(data) {
            console.log(data);
            savedSets.value = data;
        })
        .catch(function(error) {
            console.error("Error: ", error);
        });
}
onMounted(() => {
    fetchSavedFlashcardSets();
})

// create new flashcard set
onMounted(() => {
    let newFlashcardSetForm = document.getElementById("newFlashcardSetForm");
    
    newFlashcardSetForm.addEventListener("submit", (e) => {
        e.preventDefault();
        console.log("Form submitted!");
        let setName = document.getElementsByName("name")[0].value;
        let setDescription = document.getElementsByName("description")[0].value;

        let username = localStorage.getItem("username");
        if (username == null) {
            alert("Please log in to create a flashcard set!");
            return;
        }

        if (setName == "") {
            alert("Set Name cannot be empty!");
        } else {

            console.log(
                `This form has a setName of ${setName} and setDescription of ${setDescription}`,
            );

            let flashcard_deck = 
            {
                name: setName,
                description: setDescription,
            }

            let requestBody = {
                username: username,
                flashcard_deck: flashcard_deck,
            };

            console.log("Request body: ", requestBody);

            fetch("/create-flashcard-deck", {
                method: "POST",
                body: JSON.stringify(requestBody),
                headers: {
                    "Content-Type": "application/json",
                },
            })
                .then((response) => {
                    if (!response.ok) {
                        errorMessage.value = "Invalid flashcard set name.";
                        setTimeout(() => {
                            errorMessage.value = "";
                        }, 5000);
                        // throw new Error("Not ok");
                    }
                    else {
                        console.log("Flashcard set created successfully!");
                        savedSets.value.push(flashcard_deck);
                        toggleVisibility();
                        document.getElementById("newFlashcardSetForm").reset();
                    }
                })
                .catch((error) => {
                    console.error("Error: ", error);
                });
        
    }  
    });  
});

// create new flashcard
onMounted(() => 
{
    document.getElementById("newFlashcardForm").addEventListener("submit", (e) => {
    e.preventDefault();

    let front = document.getElementsByName("front")[0].value;
    let back = document.getElementsByName("back")[0].value;
    // let deckName = localStorage.getItem("currentDeck");

    let username = localStorage.getItem("username");
    if (username == null) {
        alert("Please log in to create a flashcard!");
        return;
    }

    if (front == "" || back == "") {
        alert("Front and Back cannot be empty!");
    } else {
        flashcards.value.push({ flashcard_front: front, flashcard_back: back });            
        document.getElementById("newFlashcardForm").reset();
    }
    });
})

// open new flashcard form
function openNewFlashcardForm() {
    isFlashcardFormVisible.value = true;
}

// close new flashcard form and save flashcards to the database
function closeFormAndSaveFlashcards() {
    isFlashcardFormVisible.value = false;

    if (localStorage.getItem("username") == null) {
        alert("Please log in to create a flashcard!");
        return;
    }

    let createFlashcardsData = {
        username: localStorage.getItem("username"),
        flashcard_deck_name: localStorage.getItem("currentDeck"),
        flashcards: flashcards.value,
    }

    fetch(`/create-flashcards`, {
        method: "POST",
        body: JSON.stringify(createFlashcardsData),
        headers: {
            "Content-Type": "application/json",
        },
    })
        .then(function(response) {
            return response.json();
        })
        .then(function(data) {
            console.log(data);
            console.log("Flashcards saved successfully!");
        })
        .catch(function(error) {
            console.error("Error: ", error);
        });
}
</script>

<template>
    <Page direction="row" justifyContent="space-between" alignTimes="auto">
        <div class="sidebar">
            <button class="sticky-button" :set="isFlashcardSetFormVisible" @click="toggleVisibility">Create new flashcard set</button>
            <h3 id="saved-sets-header" style="color: var(--color-primary)">SAVED SETS</h3>
            <div id="saved-sets">
                <div v-for="set in savedSets" 
                    :key="set.id" 
                    class="flashcard-set"
                    style = "cursor: pointer;"
                    @click="selectSet(set)"
                >
                    {{ set.name }}
                </div>
                <FlashcardSetOptions 
                    v-if="selectedSet" 
                    :set="selectedSet" 
                    :deckName=selectedSet.name 
                    :isSetSelected="true" 
                    :functionOnClick="openNewFlashcardForm"/>
            </div>
        </div>
        <div class="flashcards-main">
            <template v-for="flashcard in flashcards" v-if="flashcards">
                <Flashcard 
                    :front=flashcard.flashcard_front 
                    :back=flashcard.flashcard_back>
                </Flashcard>
            </template>
        </div>
    </Page>

    <form class="form-popup" id="newFlashcardSetForm" :class="{ 'show': isFlashcardSetFormVisible }" v-show="isFlashcardSetFormVisible">
        <h1>New Flashcard Set</h1>

        <p v-if="errorMessage" style="color: red;">{{ errorMessage }}</p>
        <label for="name"><b>Name</b></label>
        <input type="text" placeholder="Enter Name" name="name" required>

        <label for="description"><b>Description</b></label>
        <input type="text" placeholder="Enter Description" name="description">

        <button type="submit" class="btn">Create</button>
        <button type="button" class="btn cancel" @click="toggleVisibility">Close</button>
    </form>

    <form class="form-popup" id="newFlashcardForm" :class="{ 'show': isFlashcardFormVisible }" v-show="isFlashcardFormVisible">
        <h1>New Flashcard</h1>

        <label for="front"><b>Front</b></label>
        <input type="text" name="front" required>

        <label for="back"><b>Back</b></label>
        <input type="text" name="back">

        <button type="submit" class="btn">Create</button>
        <button type="button" class="btn cancel" @click="closeFormAndSaveFlashcards">Close</button>
    </form>
</template>

<style scoped>
.sidebar {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    width: 20%;
    gap: 1rem;
}

#saved-sets {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    gap: 0.5rem;
}

#saved-sets-header {
    color: var(--color-primary);
    font-weight: bold;
    letter-spacing: 0.4rem;
}

.sticky-button {
    position: sticky;
    top: 0;
    min-width: 14em;
}

.flashcards-main {
    width: 80%;
    height: 100%;
    border-color: var(--color-primary);
    border-width: 0.2em;
    border-style: solid;
    border-radius: 1em;
    padding: 1em;
    min-height: 5em;
    font-size: 1.5rem;
    display: flex;
    gap: 1em;
    flex-wrap: wrap;
}
</style>