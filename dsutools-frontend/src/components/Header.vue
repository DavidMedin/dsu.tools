<template>
    <header>
        <div id="name">
            <h1 class="dsu-blue">DSU</h1>
            <h1 class="dsu-blue">TOOLS</h1>
        </div>
    </header>
</template>

<style scoped>
#name {
    display: flex;
    flex-direction: column;
    padding-top: 1rem;
    padding-left: 1rem;
}
header {
    flex-grow: 0;

    display: flex;
    justify-content: space-between;
}

.dsu-blue {
    padding: 0;
    line-height: 1em;
}

header > * {
    align-self: center;
}
</style>
