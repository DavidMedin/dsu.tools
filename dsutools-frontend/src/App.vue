<!-- This file will be imported by main.js by index.html. -->

<script setup>
import Page from "./components/Page.vue";
import ToolGrid from "./components/ToolGrid.vue";
import ToolItem from "./components/ToolItem.vue";
import ToolButton from "./components/ToolButton.vue";
import QuickLinks from "./components/QuickLinks.vue";
import Calculator from "./components/Calculator.vue";
import Vitamin from "./components/Vitamin.vue";
</script>

<template>
    <Page>
        <div id="center-page">
            <nav id="left-nav">
                <QuickLinks />
            </nav>
            <div>
                <ToolGrid>
                    <ToolItem tool_name="Calculator">
                        <Calculator />
                    </ToolItem>
                    <ToolButton
                        tool_name="Vitamin Calculator"
                        tool_link="/vitamincalculator.html"
                    />
                    <ToolButton
                        tool_name="Study Buddy"
                        tool_link="/flashcards.html"
                    />
                    <ToolButton
                        tool_name="Color Calculator"
                        tool_link="/colors.html"
                    />
                </ToolGrid>
            </div>

            <nav id="right-nav"></nav>
        </div>
    </Page>
</template>

<style scoped>
#center-page {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    flex-grow: 1;
    padding-top: 1rem;
}
</style>
