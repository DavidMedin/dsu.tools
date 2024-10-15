<script setup>
import { ref, reactive } from 'vue'

const curNumber = ref('') //what is currently being typed
const cacheNum = ref('') //used if an operator has been selected to store prev curNumber
const Operator = Object.freeze({ //used to store which operator is currently happening (enum)
  NONE: 'n/a',
  DONE: '=',
  ADD: '+',
  SUB: '-',
  MUL: '*',
  DIV: '/',
  EXP: '^',
  LOG: 'l10'
})

var curOp = Operator.NONE
var cacheOp = Operator.NONE
var clearText = false

function Append(input)
{
  if(curOp === Operator.DONE)
  {
    ClearMem()
    clearText = false
  }
  else if(clearText)
  {
    Clear()
    clearText = false
  }
  
  //adds input at the end of the string
  console.log(input)
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

function Delete()
{
  curNumber.value = curNumber.value.substring(0, curNumber.value.length-1)
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
  var numA = parseInt(cacheNum.value)
  var numB = parseInt(curNumber.value)
  var calculation = 0
  var doCacheNum = true

  //check to see if equals is getting pressed over and over
  if(curOp === Operator.DONE)
  {
    curOp = cacheOp
    doCacheNum = false
  }
  else
  {
    doCacheNum = true
  }
  
  //do equals
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
  else if(curOp === Operator.MUL)
  {
    calculation = numA * numB
  }
  else if(curOp === Operator.DIV)
  {
    calculation = numA / numB
  }
  else if(curOp === Operator.EXP)
  {
    calculation = numA ** numB
  }
  else if(curOp === Operator.LOG)
  {
    calculation = Math.log10(numA)
  }

  if(doCacheNum)
    cacheNum.value = numB
  cacheOp = curOp
  curOp = Operator.DONE
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
    <button class="child-flex" @click="Delete()"> DEL </button>
    <button class="child-flex" @click="Operation(Operator.ADD)"> + </button>
    <button class="child-flex" @click="Operation(Operator.SUB)"> - </button>
    <button class="child-flex" @click="Operation(Operator.MUL)"> * </button>
    <button class="child-flex" @click="Operation(Operator.DIV)"> / </button>
    <button class="child-flex" @click="Operation(Operator.EXP)"> ^ </button>
    <button class="child-flex" @click="Operation(Operator.LOG)"> log10 </button>
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