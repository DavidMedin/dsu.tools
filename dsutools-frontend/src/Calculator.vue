<script setup>
import { ref, reactive } from 'vue'
//import { useMath } from '@vueuse/math'

const curNumber = ref('0') //what is currently being typed
const cacheNum = ref('') //used if an operator has been selected to store prev curNumber
const Operator = Object.freeze({ //used to store which operator is currently happening (enum)
  NONE: 'n/a',
  DONE: '=',
  ADD: '+',
  SUB: '-',
  MUL: '*',
  DIV: '/',
  EXP: '^',
  LOG: 'l10',
  LN: 'ln',
  LGB: 'xlogy',
  ABS: '|',
  REC: '1/x',
  NEG: '+/-'
})

var curOp = Operator.NONE
var cacheOp = Operator.NONE
var clearText = false
var numberPressed = false

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
  if(curNumber.value === '0' && input != '.')
  {
    curNumber.value = input
  }
  else if (input === Math.PI || input === Math.E)
  {
    curNumber.value = input
  }
  else
  {
    curNumber.value += input
  }
  numberPressed = true
}

function Clear()
{
  curNumber.value = '0'
}

function ClearMem()
{
  curNumber.value = '0'
  curOp = Operator.NONE
}

function Delete()
{
  curNumber.value = curNumber.value.substring(0, curNumber.value.length-1)
}

function Operation(input)
{
  //some operations are done when the button is pressed,
  //so they need to be checked before doing the equals
  if(input === Operator.LOG)
  {
    curNumber.value = Math.log10(curNumber.value)
  }
  else if(input === Operator.LN)
  {
    curNumber.value = Math.log(curNumber.value)
  }
  else if (input === Operator.ABS && curNumber.value < 0)
  {
    curNumber.value *= -1
  }
  else if (input === Operator.NEG)
  {
    curNumber.value = curNumber.value * -1  
  }
  else if (input === Operator.REC)
  {
    curNumber.value = 1 / curNumber.value
  }
  else if(curOp !== Operator.NONE && numberPressed )
  {
    Equals()
  }
  cacheOp = curOp
  curOp = input
  cacheNum.value = curNumber.value
  clearText = true;
  numberPressed = false
}

function Equals()
{
  var numA = parseFloat(cacheNum.value)
  var numB = parseFloat(curNumber.value)
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
  else if(curOp === Operator.LGB)
  {
    calculation = Math.log(numA) / Math.log(numB)
  }

  if(doCacheNum)
    cacheNum.value = numB
  cacheOp = curOp
  curOp = Operator.DONE
  console.log('curOP: ', curOp)
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
    <button class="child-flex" @click="Operation(Operator.NEG)"> -/+ </button>
    <button class="child-flex" @click="Operation(Operator.ADD)"> + </button>
    <button class="child-flex" @click="Operation(Operator.SUB)"> - </button>
    <button class="child-flex" @click="Operation(Operator.MUL)"> * </button>
    <button class="child-flex" @click="Operation(Operator.DIV)"> / </button>
    <button class="child-flex" @click="Operation(Operator.EXP)"> ^ </button>
    <button class="child-flex" @click="Append(Math.PI)"> π </button>
    <button class="child-flex" @click="Append(Math.E)"> e </button>
    <button class="child-flex" @click="Operation(Operator.LOG)"> log10 </button>
    <button class="child-flex" @click="Operation(Operator.LN)"> ln </button>
    <button class="child-flex" @click="Operation(Operator.LGB)"> x log base y </button>
    <button class="child-flex" @click="Operation(Operator.REC)"> 1/x </button>
    <button class="child-flex" @click="Operation(Operator.ABS)"> |x| </button>
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