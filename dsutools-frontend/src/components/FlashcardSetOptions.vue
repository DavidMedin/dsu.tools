<script setup>
import { onMounted, ref } from "vue";

const isFlashcardFormVisible = ref(false);

function openNewFlashcardForm() {
    isFlashcardFormVisible.value = true;
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

// close new flashcard form and save flashcards to the database
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
</script>

<template>
    <button class="sticky-button" v-show="isFlashcardSetSelected" @click="openNewFlashcardForm">Create new flashcard</button>
    
    <form class="form-popup" id="newFlashcardForm" :class="{ 'show': isFlashcardFormVisible }">
        <h1>New Flashcard</h1>

        <label for="front"><b>Front</b></label>
        <input type="text" name="front" required>

        <label for="back"><b>Back</b></label>
        <input type="text" name="back">

        <button type="submit" class="btn">Create</button>
        <button type="button" class="btn cancel" @click="closeNewFlashcardForm">Close</button>
    </form>
</template>

<style scoped>
.form-popup {
  display: none;
}

.form-popup.show {
    display: flex;
}
</style>
