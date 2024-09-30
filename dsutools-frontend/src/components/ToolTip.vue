<!-- Warning! In order to use this component, put this inside the tooltip'd element, and give it the class "tooltip-haver"! -->
<script setup>
</script>


<template>
<div class="tooltip">
Copy to Clipboard
</div>
</template>


<style>
.tooltip {
    display:none;
    position:absolute;
    margin-bottom: 4.5rem;
    background-color: var(--color-surface-lvl-1);
    border-radius: 0.25rem;
    padding: 0.25rem;

    -moz-user-select: none;
    -webkit-user-select: none;
    -ms-user-select: none;
    user-select: none;
    -o-user-select: none;
    pointer-events: none;
}
.tooltip-haver:hover > .tooltip {
    display:inline;
}
</style>
