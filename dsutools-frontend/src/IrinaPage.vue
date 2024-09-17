<script setup>
</script>

<template>
    <main>
        <div>Tool1</div>
        <div>Tool2</div>
        <div>Tool3</div>
        <div>Tool4</div>
        <div>Tool5</div>
    </main>
</template>

<style scoped>
    * {
        font-family: 'Courier New', Courier, monospace;
        color: black;
    }

    main {
        display: flex;
        margin: 3rem;
        gap: 2rem;
        align-items: center;
        justify-content: center;
        flex-wrap: wrap;
    }
    div {
        background-color: burlywood;
        width: 20rem;
        height: 20rem;
        border-radius: 15%;
        text-align: center;
    }
</style>