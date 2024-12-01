<script setup>
import { inject, computed } from "vue";
import CopyableText from "../CopyableText.vue";

function truncToTwoDecimalPlaces(n) {
    return Math.floor(n * 100) / 100;
}

const props = defineProps({
    color: { type: Array, required: true },
    color_space_index: { type: Number, required: true },
});
const color_spaces = inject("ColorSpaces");
const toRGBHex = computed(
    () => color_spaces[props.color_space_index].conversions.toRGBHex,
);
const rgb_hex = computed(() => toRGBHex.value(props.color));

const hct = computed(() =>
    color_spaces[0].conversions
        .fromRGBHex(rgb_hex.value)
        .map(truncToTwoDecimalPlaces),
);
const rgb = computed(() =>
    color_spaces[1].conversions.fromRGBHex(rgb_hex.value),
);
</script>

<template>
    <table class="color-output">
        <thead>
            <tr>
                <th scope="col">Color Space</th>
                <th scope="col" colspan="3">Components</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>
                    <CopyableText :text="'#' + rgb_hex" />
                </td>
                <td><CopyableText :text="rgb[0]" /></td>
                <td><CopyableText :text="rgb[1]" /></td>
                <td><CopyableText :text="rgb[2]" /></td>
            </tr>
            <tr>
                <td>
                    <CopyableText
                        :text="
                            'rgb(' + rgb[0] + ',' + rgb[1] + ',' + rgb[2] + ')'
                        "
                    />
                </td>
                <td><CopyableText label="Red: " :text="rgb[0]" /></td>
                <td><CopyableText label="Green: " :text="rgb[1]" /></td>
                <td><CopyableText label="Blue: " :text="rgb[2]" /></td>
            </tr>

            <tr>
                <td>
                    <CopyableText
                        :text="
                            'hct(' + hct[0] + ',' + hct[1] + ',' + hct[2] + ')'
                        "
                    />
                </td>
                <td><CopyableText label="Hue: " :text="hct[0]" /></td>
                <td><CopyableText label="Chroma: " :text="hct[1]" /></td>
                <td><CopyableText label="Tone: " :text="hct[2]" /></td>
            </tr>
        </tbody>
    </table>
</template>

<style scoped>
.color-output {
    /* display: flex; */
    /* flex-direction: column; */
    /* align-items: center; */
    /* justify-content: center; */
}
</style>
