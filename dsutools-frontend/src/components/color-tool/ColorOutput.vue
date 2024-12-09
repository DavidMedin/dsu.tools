<script setup>
import { inject, computed } from "vue";
import {
    serialize,
    to as convert,
    sRGB_Linear,
    toGamut,
    get,
} from "colorjs.io/fn";
import {
    truncToTwoDecimalPlaces,
    fmt_convert,
    to_hex,
} from "../../colorUtils.js";

import CopyableText from "../CopyableText.vue";

const props = defineProps({
    color: { type: Object, required: true },
    color_space_index: { type: Number, required: true },
});
const color_spaces = inject("ColorSpaces");
const rgb_hex = computed(() => to_hex(props.color));

let spaces = computed(() => {
    let spaces = [];
    for (let i = 0; i < color_spaces.length; i++) {
        let space = color_spaces[i];
        let space_name = space.name;

        // Produce the string like "rgb(1,3.2,20)"
        let func_name = space_name + "(";
        let fmted_color = fmt_convert(props.color, space);
        for (const coord of fmted_color) {
            func_name += coord + ",";
        }
        func_name = func_name.substr(0, func_name.length - 1);
        func_name += ")";

        let coords = [];
        for (const [index, coord] of Object.entries(space.coords)) {
            coords.push({
                name: `${coord.name}: `,
                value: truncToTwoDecimalPlaces(
                    get(props.color, [space, index]),
                ),
            });
        }

        spaces.push({
            func: func_name,
            coords: coords,
        });
    }
    return spaces;
});
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
            <tr v-for="space in spaces">
                <td>
                    <CopyableText :text="space.func" />
                </td>
                <td v-for="coord in space.coords">
                    <CopyableText :label="coord.name" :text="coord.value" />
                </td>
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
