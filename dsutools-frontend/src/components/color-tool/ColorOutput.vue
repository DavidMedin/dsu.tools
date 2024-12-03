<script setup>
import { inject, computed } from "vue";
import { serialize, to as convert, sRGB_Linear, toGamut } from "colorjs.io/fn";
import {
    truncToTwoDecimalPlaces,
    fmt_convert,
    to_hex,
} from "../../colorUtils.js";

// import Color from "colorjs.io";
import CopyableText from "../CopyableText.vue";

const props = defineProps({
    color: { type: Object, required: true },
    color_space_index: { type: Number, required: true },
});
const color_spaces = inject("ColorSpaces");
const rgb_hex = computed(() => to_hex(props.color));

const hct = computed(() => fmt_convert(props.color, color_spaces[0]));
const rgb = computed(() => fmt_convert(props.color, color_spaces[1]));
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
                    <CopyableText :text="rgb_hex" />
                </td>
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
