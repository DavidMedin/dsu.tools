<script setup>
import Flashcard from "./components/Flashcard.vue";
import Page from "./components/Page.vue";

import { onMounted, ref } from "vue";

const isFormVisible = ref(false);
const isFlashcardFormVisible = ref(false);
const flashcards = ref([]);
const isFlashcardSetSelected = ref(false);

function openNewFlashcardSetForm() {
    isFormVisible.value = true;
}

function closeNewFlashcardSetForm() {
    isFormVisible.value = false;
}

function openNewFlashcardForm() {
    isFlashcardFormVisible.value = true;
}

function closeNewFlashcardForm() {
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

// display the saved flashcard decks in the sidebar
function addFlashcardSetToSidebar(flashcardSet) {
    let savedSets = document.getElementById("saved-sets");
    let flashcardSetElement = document.createElement("div");
    flashcardSetElement.textContent = flashcardSet.name;
    flashcardSetElement.classList.add("flashcard-set");
    flashcardSet.id = flashcardSet.name;
    flashcardSetElement.style.cursor = "pointer";
    flashcardSetElement.addEventListener("click", () => {
        isFlashcardSetSelected.value = true;
        fetchDecksFlashcards(flashcardSet.name);
        localStorage.setItem("currentDeck", flashcardSet.name);
    });
    savedSets.appendChild(flashcardSetElement);
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
function displaySavedFlashcardSets() {
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
            for (let set of data) {
                addFlashcardSetToSidebar(set);
            }
        })
        .catch(function(error) {
            console.error("Error: ", error);
        });
}

// create new flashcard
onMounted(() => 
{document.getElementById("newFlashcardForm").addEventListener("submit", (e) => {
    e.preventDefault();

    let front = document.getElementsByName("front")[0].value;
    let back = document.getElementsByName("back")[0].value;
    let deckName = localStorage.getItem("currentDeck");

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
});})

onMounted(() => {
    displaySavedFlashcardSets();

    let newFlashcardSetForm = document.getElementById("newFlashcardSetForm");
    
    newFlashcardSetForm.addEventListener("submit", (e) => {
        e.preventDefault();
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

            let requestBody = {
                username: username,
                flashcard_deck:
                {
                    name: setName,
                    description: setDescription,
                }
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
                        const message = document.createElement("p");
                        message.textContent = "Invalid flashcard set name.";
                        message.style.color = "red";
                        document.getElementById("newFlashcardSetForm").appendChild(message);
                        
                        setTimeout(() => {
                            message.remove();
                        }, 5000);
                        // throw new Error("Not ok");
                    }
                    else {
                        console.log("Flashcard set created successfully!");
                        addFlashcardSetToSidebar(requestBody.flashcard_deck);
                    }
                })
                .catch((error) => {
                    console.error("Error: ", error);
                });
                
                document.getElementById("newFlashcardSetForm").reset();
                // closeNewFlashcardSetForm();
        }
    })
})

</script>

<template>
    <form class="form-popup round-box" id="newFlashcardSetForm" :class="{ 'show': isFormVisible }">
        <h1>New Flashcard Set</h1>

        <label for="name"><b>Name</b></label>
        <input type="text" placeholder="Enter Name" name="name" required>

        <label for="description"><b>Description</b></label>
        <input type="description" placeholder="Enter Description" name="description">

        <button type="submit" class="btn">Create</button>
        <button type="button" class="btn cancel" @click="closeNewFlashcardSetForm">Close</button>
    </form>
    <form class="form-popup round-box" id="newFlashcardForm" :class="{ 'show': isFlashcardFormVisible }">
        <h1>New Flashcard</h1>

        <label for="front"><b>Front</b></label>
        <input type="text" name="front" required>

        <label for="back"><b>Back</b></label>
        <input type="text" name="back">

        <button type="submit" class="btn">Create</button>
        <button type="button" class="btn cancel" @click="closeNewFlashcardForm">Close</button>
    </form>
    <Page direction="row" justifyContent="space-between" alignTimes="auto">
        <div class="sidebar">
            <button class="sticky-button" @click="openNewFlashcardSetForm">Create new flashcard set</button>
            <button class="sticky-button" v-show="isFlashcardSetSelected" @click="openNewFlashcardForm">Create new flashcard</button>

            <h3 id="saved-sets-header" style="color: var(--color-primary)">SAVED SETS</h3>
            <div id="saved-sets">
            </div>
        </div>
        <div class="flashcards-main">
            <template v-for="flashcard in flashcards" v-if="flashcards">
                <Flashcard :front=flashcard.flashcard_front :back=flashcard.flashcard_back></Flashcard>
            </template>
        </div>
    </Page>
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

#saved-sets-header {
    color: var(--color-primary);
    font-weight: bold;
    letter-spacing: 0.4rem;
}

.sticky-button {
    position: sticky;
    top: 0;
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

/* The popup form - hidden by default */
.form-popup {
  display: none;
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 30%;
  height: 60%;
  z-index: 9;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border-color: var(--color-primary);
  border-width: 0.2em;
  border-style: solid;
  opacity: 0;
  transition: opacity 0.5s ease;
}

.form-popup.show {
    display: flex;
    opacity: 1;
}

#newFlashcardSetForm,
#newFlashcardForm {
    box-shadow: 0 0.25rem 0.5rem var(--color-primary), 0 0.375rem 1.25rem 0 var(--color-primary);
}

#newFlashcardSetForm input,
#newFlashcardForm input {
    width: 100%;
    padding: 0.5em;
    margin-bottom: 1em;
    border: 1px solid var(--color-primary);
    border-radius: 4px;
    box-sizing: border-box;
    background-color: var(--color-background-mute);
}

#newFlashcardSetForm button,
#newFlashcardForm button {
    padding: 0.5em 1em;
    background-color: darken(var(--color-primary), 10%);
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer; 
}

#newFlashcardSetForm button:hover,
#newFlashcardForm button:hover {
    background-color: var(--color-primary);
}

</style>