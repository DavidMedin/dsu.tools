<script setup>
import { ref } from 'vue' 
import { reactive } from 'vue'

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

//

</script>

<template>
  <button @click="increment">Count is: {{ count }}</button>
</template>