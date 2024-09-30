<script setup>
import { inject } from "vue";
import CopyableText from "../CopyableText.vue";

defineProps({
    color: { type: Array, required: true },
    color_space_index: { type: Number, required: true },
});
const color_spaces = inject("ColorSpaces");
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
                    <CopyableText
                        :text="
                            '#' +
                            color_spaces[
                                color_space_index
                            ].conversions.toRGBHex(color)
                        "
                    />
                </td>
                <td><CopyableText text="Red: 1" /></td>
                <td><CopyableText text="Green: 2" /></td>
                <td><CopyableText text="Blue: 3" /></td>
            </tr>
            <tr>
                <td><CopyableText text="RGB" /></td>
                <td><CopyableText text="Red: 1" /></td>
                <td><CopyableText text="Green: 2" /></td>
                <td><CopyableText text="Blue: 3" /></td>
            </tr>

            <tr>
                <td><CopyableText text="HCT" /></td>
                <td><CopyableText text="Hue: 2" /></td>
                <td><CopyableText text="Chroma: 3" /></td>
                <td><CopyableText text="Tone: 4" /></td>
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
