<script setup>
import Box from "./components/Box.vue";
import Header from "./Header.vue";
import Flashcard from "./components/Flashcard.vue";
</script>

<template>
    <main>
        <h1>Welcome</h1>
        <!-- <Box id="dsu-tools" tool_name="dsu-tools"><Header style="color: var(--pink-text)">DSU TOOLS</Header></Box>
        <Box id="calculator" tool_name="calculator"><Header>CALCULATOR</Header></Box>
        <Box id="events" tool_name="events"><Header>EVENTS</Header></Box>
        <Box id="puzzle" tool_name="puzzle"><Header>PUZZLE</Header></Box>
        <div class="scientific-calculators">
            <Box id="vitamins-calculator" tool_name="vitamins-calculator"><Header>VITAMINS CALCULATOR</Header></Box>
            <Box id="molecules-calculator" tool_name="molecules-calculator"><Header>MOLECULES CALCULATOR</Header></Box>
        </div>
        <Box id="todays-menu" tool_name="todays-menu"><Header>TODAYS MENU</Header></Box>
        <Box id="flashcards"  tool_name="flashcards"><Header>FLASHCARDS</Header></Box> -->
    </main>
</template>

<style scoped>
    * {
        font-family: 'Courier New', Courier, monospace;
        color: black;
        background-color: var(--dark-blue);
    }

    main {
        display: flex;
        align-items: center;
        justify-content: center;
        flex-wrap: wrap;
        position: absolute;
    }

    .scientific-calculators {
        display: flex;
        flex-direction: column;
        margin: 0;
    }

    #dsu-tools,
    #calculator,
    #events,
    #vitamins-calculator,
    #molecules-calculator 
    {
        height: 20rem;
    } 

    #dsu-tools, 
    #puzzle
    {
        width: 20rem;
    }

    #calculator,
    #vitamins-calculator,
    #molecules-calculator
    {
        width: 30rem;
    }
    #events {
        width: 40rem;
    }

    #puzzle,
    #todays-menu,
    #flashcards,
    .scientific-calculators {
        height: 40rem;
    }

    #dsu-tools {
        background-color: var(--white);
        color: var(--pink-text);
    }

    #calculator,
    #events,
    #molecules-calculator {
        background-color: var(--pink-background);
    }

    #puzzle, 
    #vitamins-calculator    {
        background-color: var(--blue);
    }

    #flashcards {
        background-color: var(--light-pink-background);
        width: 19.5rem;
    }

    #todays-menu{
        background-color: var(--blue);
        width: 19.5rem;
    }
</style>