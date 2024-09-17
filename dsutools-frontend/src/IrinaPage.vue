<script setup>
</script>

<template>
    <main>
        <div id="dsu-tools"           class="box">DSU TOOLS</div>
        <div id="calculator"          class="box">CALCULATOR
            <!-- <i class="arrow right"></i> -->
        </div>
        <div id="events"              class="box">EVENTS</div>
        <div id="puzzle"              class="box">PUZZLE</div>
        <div class="scientific-calculators">
            <div id="vitamins-calculator" class="box">VITAMINS CALCULATOR</div>
            <div id="molecules-calculator">MOLECULES CALCULATOR</div>
        </div>
        <div id="todays-menu"         class="box">TODAYS MENU</div>
        <div id="flashcards"          class="box">FLASHCARDS</div>
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

    div {
        display: flex;
        border-radius: 5rem;
        justify-content: center;
        align-items: center;
        margin: 0.5rem 1rem 0.5rem 1rem;
        font-size: 2rem;
        color: var(--white);
    }

    .scientific-calculators {
        display: flex;
        flex-direction: column;
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
    .scientific-calculators {
        margin: 0;
    }
</style>