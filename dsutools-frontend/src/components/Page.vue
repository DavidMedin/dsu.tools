<script setup>
// maybe header and footer should be in this file?
import Footer from "./Footer.vue";
import Header from "./Header.vue";

const props = defineProps({
  direction: {
    type: String,
    default: 'row', // Default direction
  },
  justifyContent: {
    type: String,
    default: 'center',
  },
  alignTimes: {
    type: String,
    default: 'center',
  }
});
</script>

<template>
    <div id="everything">
        <Header />
        <main :style="{ flexDirection: direction, justifyContent: justifyContent, alignTimes: alignTimes}">
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
    flex-grow: 1;
}
</style>
