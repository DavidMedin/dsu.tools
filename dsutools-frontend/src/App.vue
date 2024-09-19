<!-- This file will be imported by main.js by index.html. -->

<script setup>
import ToolGrid from "./components/ToolGrid.vue";
import ToolItem from "./components/ToolItem.vue";
import ToolButton from "./components/ToolButton.vue";
import Footer from "./components/Footer.vue";
import Header from "./components/Header.vue";
</script>

<template>
    <div id="everything">
        <Header />

        <main>
            <p id="pregrid-text">Hello. Here are some tools.</p>
            <ToolGrid>
                <ToolItem
                    tool_name="Calculator"
                    style="grid-row: 1 / 3; grid-column: 1 / 2"
                />
                <ToolItem tool_name="Sodexo Menu" style="grid-column: 2 / 4" />
                <ToolButton tool_name="Study Buddy" style="grid-column: 2/3" />
                <ToolItem
                    tool_name="DSU Events"
                    style="grid-row: 2/4; grid-column: 3/4"
                />
                <ToolItem tool_name="Quick Links" style="grid-column: 1/3">
                    <ul>
                        <li>
                            <a href="https://dsu.edu"> DSU Website </a>
                        </li>
                        <li>
                            <a href="https://mydsu.dsu.edu">My DSU</a>
                        </li>
                        <li>
                            <a href="https://d2l.sdbor.edu/d2l/home">D2L</a>
                        </li>
                    </ul>
                </ToolItem>
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
#pregrid-text {
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    text-align: center;
}
</style>
