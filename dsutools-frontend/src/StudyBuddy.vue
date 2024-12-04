<script setup>
import Flashcard from "./components/Flashcard.vue";
import Page from "./components/Page.vue";

import { onMounted, ref } from "vue";

const isFormVisible = ref(false);

function openNewFlashcardSetForm() {
    isFormVisible.value = true;
}

function closeNewFlashcardSetForm() {
    isFormVisible.value = false;
}

onMounted(() => {
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
    <Page direction="row" justifyContent="space-between" alignTimes="auto">
        <div class="sidebar">
            <button class="sticky-button" @click="openNewFlashcardSetForm">Create new flashcard set</button>

            <h3 id="saved-sets-header" style="color: var(--color-primary)">SAVED SETS</h3>
            <div id="saved-sets">
            </div>
        </div>
        <div class="flashcards-main">
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

#newFlashcardSetForm {
    box-shadow: 0 0.25rem 0.5rem var(--color-primary), 0 0.375rem 1.25rem 0 var(--color-primary);
}

#newFlashcardSetForm input {
    width: 100%;
    padding: 0.5em;
    margin-bottom: 1em;
    border: 1px solid var(--color-primary);
    border-radius: 4px;
    box-sizing: border-box;
    background-color: var(--color-background-mute);
}

#newFlashcardSetForm button {
    padding: 0.5em 1em;
    background-color: darken(var(--color-primary), 10%);
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer; 
}

#newFlashcardSetForm button:hover {
    background-color: var(--color-primary);
}

</style>