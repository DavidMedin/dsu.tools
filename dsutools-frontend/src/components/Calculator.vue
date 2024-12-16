<script setup>
import { ref, reactive } from 'vue'
import { Big } from 'big.js'
//import { useMath } from '@vueuse/math'

const curNumber = ref('0') //what is currently being typed //used if an operator has been selected to store prev curNumber
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
  SQRT: 'sqrt',
  ABS: '|',
  REC: '1/x',
  NEG: '+/-'
})

var curOp = Operator.NONE
var cacheOp = Operator.NONE
var clearText = false
var numberPressed = false
var piOrEPressed = false
const decimalPlaces = 5;
var realValue = new Big('0');
var cacheNum = new Big('0');

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
  if(curNumber.value === '0' && input != '.' && input !== Math.PI && input !== Math.E)
  {
    realValue = input
    curNumber.value = input
    piOrEPressed = false
  }
  else if (input === Math.PI || input === Math.E)
  {
    realValue = input
    curNumber.value = input.toFixed(decimalPlaces)
    piOrEPressed = true
  }
  else
  {
    //prevents numbers from appending to pi/e
    if(curNumber.value === Math.PI || curNumber.value === Math.E)
    {
      realValue = input
      curNumber.value = input
    }
    else
    {
      realValue += input
      curNumber.value += input
    }
  }
  if(Math.log10(realValue) > decimalPlaces)
  {
    curNumber.value = (Number(realValue)).toPrecision(decimalPlaces)
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
  if(curNumber.value.length === 0)
    curNumber.value = '0'
}

function Operation(input)
{
  if(curOp === Operator.DONE)
  {
    ClearMem();
  }
  //some operations are done when the button is pressed,
  //so they need to be checked before doing the equals
  if(input === Operator.LOG)
  {
    realValue = Math.log10(realValue)
    curNumber.value = realValue.toFixed(decimalPlaces);
  }
  else if(input === Operator.LN)
  {
    realValue = Math.log(realValue)
    curNumber.value = realValue.toFixed(decimalPlaces);
  }
  else if (input === Operator.ABS && curNumber.value < 0)
  {
    realValue *= -1
    curNumber.value = realValue
  }
  else if (input === Operator.NEG)
  {
    realValue *= -1
    curNumber.value = realValue
  }
  else if (input === Operator.REC)
  {
    realValue = 1 / realValue
    curNumber.value = realValue.toFixed(decimalPlaces)
  }
  else if (input === Operator.SQRT)
  {
    realValue = Math.sqrt(curNumber.value)
    curNumber.value = realValue.toFixed(decimalPlaces);
  }
  else if(curOp !== Operator.NONE && numberPressed )
  {
    Equals()
  }
  cacheOp = curOp
  curOp = input
  cacheNum = realValue;
  clearText = true;
  numberPressed = false
}

function Equals()
{
  var numA = cacheNum
  var numB = new Big(curNumber.value)
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
    realValue = Number(numA) + Number(numB)
  }
  else if(curOp === Operator.SUB)
  {
    realValue = numA - numB
  }
  else if(curOp === Operator.MUL)
  {
    realValue = numA * numB
  }
  else if(curOp === Operator.DIV)
  {
    if(Number(numB) === 0)
      realValue = Number.NaN
    else
      realValue = numA / numB
  }
  else if(curOp === Operator.EXP)
  {
    realValue = numA ** numB
  }
  else if(curOp === Operator.LGB)
  {
    realValue = Math.log(numA) / Math.log(numB)
  }

  if(doCacheNum)
    cacheNum = numB
  cacheOp = curOp
  curOp = Operator.DONE
  if(Math.ceil(realValue) > realValue)
    curNumber.value = realValue.toFixed(decimalPlaces)
  else if(Math.log10(realValue) > decimalPlaces)
    curNumber.value = (Number(realValue)).toPrecision(decimalPlaces)
  else
    curNumber.value = realValue
  clearText = true;
}

</script>

<template>
    <p class="number-box"> {{ curNumber }} </p>
  <!-- OPERATION BUTTONS -->
  <div class="basic-ops">
        <button class="basic-operation" @click="Operation(Operator.ADD)"> + </button>
        <button class="basic-operation" @click="Operation(Operator.SUB)"> - </button>
        <button class="basic-operation" @click="Operation(Operator.MUL)"> * </button>
        <button class="basic-operation" @click="Operation(Operator.DIV)"> / </button>
        <button class="basic-operation equals" @click="Equals()"> = </button>
      </div>
    <div class="number-pad">
      <button class="child-flex" @click="Clear()"> CLR </button>
      <button class="child-flex" @click="ClearMem()"> CLM </button>
      <button class="child-flex" @click="Delete()"> DEL </button>
      <button class="child-flex" @click="Append(Math.PI)"> π </button>
      <button class="child-flex" @click="Append(Math.E)"> e </button>
      <button class="child-flex" @click="Operation(Operator.EXP)"> x<sup>y</sup> </button>
      <button class="child-flex" @click="Operation(Operator.LOG)"> log<sub>10</sub>x </button>
      <button class="child-flex" @click="Operation(Operator.LN)"> ln </button>
      <button class="child-flex" @click="Operation(Operator.LGB)"> log<sub>y</sub>x </button>
      <button class="child-flex" @click="Operation(Operator.REC)"> <sup>1</sup>&frasl;<sub>x</sub> </button>
      <button class="child-flex" @click="Operation(Operator.ABS)"> |x| </button>
      <button class="child-flex" @click="Operation(Operator.SQRT)"> &Sqrt;x </button>
    </div>
  <!-- NUMBER BUTTONS -->
   <div class="number-pad">
      <button class="number-button" @click="Append('1')"> 1 </button>
      <button class="number-button" @click="Append('2')"> 2 </button>
      <button class="number-button" @click="Append('3')"> 3 </button>
      <button class="number-button" @click="Append('4')"> 4 </button>
      <button class="number-button" @click="Append('5')"> 5 </button>
      <button class="number-button" @click="Append('6')"> 6 </button>
      <button class="number-button" @click="Append('7')"> 7 </button>
      <button class="number-button" @click="Append('8')"> 8 </button>
      <button class="number-button" @click="Append('9')"> 9 </button>
      <button class="number-button" @click="Operation(Operator.NEG)"> -/+ </button>
      <button class="number-button" @click="Append('0')"> 0 </button>
      <button class="number-button" @click="Append('.')"> . </button>
    </div>
</template>

<style>

  .number-box {
    text-align: right;
    font-size: clamp(0.3em, 2vw, 1em);
    overflow-wrap: break-word;
    width: 100%;
    height: 15%;
  }

  .number-pad {
    width: 80%;
    height: 37.5%;
    display: flex;
    font-family: monospace;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .number-button {
    width: 33%;
    height: 25%;
    font-size: large;
    text-align: center;
    padding: 0%;
    margin: 0%;
  }

  .basic-ops {
    float: right;
    display: flex;
    flex-wrap: wrap;
    text-align: center;
    width: 20%;
    height: 75%;    
    padding: 0%;
    margin: 0%;
  }

  .basic-operation {
    width: 100%;
    height: 20%;   
    padding: 0%;
    margin: 0%;
  }

  .parent-flex {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
  }

  .child-flex {
    background-color: grey;
    margin: 0%;
    padding: 0%;

    min-width: 33%;
    width: auto;
    max-width: 50%;
    
    height: auto;
    max-height: 33%;
    text-align: center;
  }

</style>