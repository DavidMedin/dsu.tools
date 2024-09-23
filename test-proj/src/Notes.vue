<script setup>
import { ref, reactive } from 'vue'
 
//**DECLARATIVE RENDERING
//reactive: a state that can trigger updates
//  - reactive only works on objects (ex. arrays, maps, set, etc.)
//ref: can take any value type and create an object that exposes an inner
//value type and create an object that exposes inner value under a .value property
//TO USE: </h1> {{ object }} </h1> or </h1> {{ refobject.value }} </h1>

//**ATTRIBUTES**
//directive: a special attribute that starts with the v- prefix.
// - argument: the part after the colon (such as `:id`)
// - to bind to a dynamic value, use `v-bind:argumentName="valueBinded"`
//    - shorthand: `:argumentName`

//**EVENT LISTENERS**
//v-on directive: used to listen to DOM events
//  - shorthand: `@argumentName="valueBinded"`

//**FORM BINDINGS**
//v-model directive: automatically syncs the <input>'s value with the bound state

//**CONDITION RENDERERS**
//v-if directive: renders something if given value is truthy
//v-else
//v-else-if

//**LIST RENDERING**
//v-for directive: render a list of elements based on a source array
/*
  <ul> <!-- unordered list -->
    <li v-for="todo in todos" :key="todo.id"> <!-- list element -->
      {{ todo.text }}
    </li>
  </ul>
*/

</script>

<template>
  <button @click="increment">Count is: {{ count }}</button>
</template>