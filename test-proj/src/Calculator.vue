<script setup>
import { ref, reactive } from 'vue'

const curNumber = ref('') //what is currently being typed
const cacheNum = ref('') //used if an operator has been selected to store prev curNumber
const Operator = Object.freeze({ //used to store which operator is currently happening (enum)
  NONE: 'n/a',
  ADD: '+',
  SUB: '-'
})
var curOp = Operator.NONE
var clearText = false

function Append(input)
{
  if(clearText)
  {
    Clear()
    clearText = false
  }
  //adds input at the end of the string
  curNumber.value += input
}

function Clear()
{
  curNumber.value = ''
}

function ClearMem()
{
  curNumber.value = ''
  curOp = Operator.NONE
}

function Operation(input)
{
  if(curOp !== Operator.NONE)
  {
    Equals()
  }
  cacheNum.value = curNumber.value
  curOp = input
  clearText = true;
}

function Equals()
{
  var numA = parseFloat(cacheNum.value)
  var numB = parseFloat(curNumber.value)
  var calculation = ''
  if(curOp === Operator.NONE)
  {
    return;
  }
  else if(curOp === Operator.ADD)
  {
    calculation = numA + numB
  }
  else if(curOp === Operator.SUB)
  {
    calculation = numA - numB
  }

  curNumber.value = calculation
  clearText = true;
}

</script>

<template>
  <!-- NUMBER BUTTONS -->
   
  <div>
    <p> {{ curNumber }} </p>
    <div class="parent-flex">
    <button class="child-flex" @click="Append('1')"> 1 </button>
    <button class="child-flex" @click="Append('2')"> 2 </button>
    <button class="child-flex" @click="Append('3')"> 3 </button>
    <button class="child-flex" @click="Append('4')"> 4 </button>
    <button class="child-flex" @click="Append('5')"> 5 </button>
    <button class="child-flex" @click="Append('6')"> 6 </button>
    <button class="child-flex" @click="Append('7')"> 7 </button>
    <button class="child-flex" @click="Append('8')"> 8 </button>
    <button class="child-flex" @click="Append('9')"> 9 </button>
    <button class="child-flex" @click="Append('0')"> 0 </button>
    <button class="child-flex" @click="Append('.')"> . </button>
  </div>
  <!-- OTHER BUTTONS -->
  <div class="parent-flex">
    <button class="child-flex" @click="Clear()"> CLR </button>
    <button class="child-flex" @click="ClearMem()"> CLM </button>
    <button class="child-flex" @click="Operation(Operator.ADD)"> + </button>
    <button class="child-flex" @click="Operation(Operator.SUB)"> - </button>
    <button class="child-flex" @click="Equals()"> = </button>
  </div>
  </div>
  
</template>

<style>

  .calculator-bar {
    max-width: 150px;
    max-height: 40px;
    background-color: whitesmoke;
    text-decoration-color: black;
  }

  .parent-flex {
    display: flex;
    max-width: 75px;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .child-flex {
    flex-grow: 2;
  }

</style>