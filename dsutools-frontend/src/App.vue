<!-- This file will be imported by main.js by index.html. -->

<script setup>
import ToolGrid from "./components/ToolGrid.vue";
import ToolItem from "./components/ToolItem.vue";
import Footer from "./components/Footer.vue";
import Header from "./components/Header.vue";
</script>

<template>
    <div id="everything">
        <Header />

        <main>
            <p>Hello. Here are some tools.</p>
            <ToolGrid>
                <ToolItem
                    tool_name="Calculator"
                    style="grid-row: 1 / 3; grid-column: 1 / 2"
                />
                <ToolItem tool_name="Sodexo Menu" />
                <ToolItem tool_name="Study Buddy" />
                <ToolItem tool_name="DSU Events" />
            </ToolGrid>
        </main>

        <Footer />
    </div>
</template>

<style scoped>
#everything {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
}
main {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    text-align: center;
}
</style>
