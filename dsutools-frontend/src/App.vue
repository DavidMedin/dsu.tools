<!-- This file will be imported by main.js by index.html. -->

<script setup>
import ToolGrid from "./components/ToolGrid.vue";
import ToolItem from "./components/ToolItem.vue";
</script>

<template>
    <header>
        <h1 class="green">dsu.tools</h1>
    </header>

    <main>
        <div id="main_div">
            <p>Hello. Here are some tools.</p>
            <ToolGrid>
                <ToolItem tool_name="Calculator" />
                <ToolItem tool_name="Your Mom's House Live Cam" />
            </ToolGrid>
        </div>
    </main>
</template>

<style scoped>
main {
    /* Center all items in this element. */
    display: flex;
    justify-content: center;
}
</style>
