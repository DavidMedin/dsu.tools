<script setup>
// maybe header and footer should be in this file?
import Footer from "./Footer.vue";
import Header from "./Header.vue";
</script>

<template>
    <div id="everything">
        <Header />
        <main>
            <slot />
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
}
</style>
