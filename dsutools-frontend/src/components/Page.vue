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
    height: 100vh; /* Full viewport height */
    width: 100vw; /* Full viewport width */
}

main {
    display: flex;
    justify-content: center;
    align-items: center;
    flex-grow: 1;
}
</style>
