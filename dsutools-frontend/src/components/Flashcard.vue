<script setup>
import { ref } from 'vue';

const props = defineProps({
  front: {
    type: String,
    required: true
  },
  back: {
    type: String,
    required: true
  }
});

const isFlipped = ref(false);

function flipCard() {
  isFlipped.value = !isFlipped.value;
}
</script>

<template>
    <div class="flip-card" @click="flipCard">
      <div :class="['flip-card-inner', { flipped: isFlipped }]">
        <div class="flip-card-front">
          <div>{{ front }}</div>
        </div>
        <div class="flip-card-back">
         <div>{{ back }}</div>
        </div>
      </div>
    </div>
</template>
  
<style scoped>
  .flip-card {
    width: 300px;
    height: 300px;
    perspective: 1000px;
  }
  
  .flip-card-inner {
    position: relative;
    width: 100%;
    height: 100%;
    text-align: center;
    transition: transform 0.6s;
    transform-style: preserve-3d;
    box-shadow: 0 4px 8px 0 rgba(0,0,0,0.2);
  }
  
  .flip-card-inner.flipped {
    transform: rotateY(180deg);
  }
  
  .flip-card-front, .flip-card-back {
    position: absolute;
    width: 100%;
    height: 100%;
    -webkit-backface-visibility: hidden;
    backface-visibility: hidden;
    border-radius: 30px;
  }
  
  .flip-card-front {
    background-color: #bbb;
    color: black;
    display: flex;
    justify-content: center;
    align-items: center;
}

.flip-card-back {
    transform: rotateY(180deg);
    background-color: #2980b9;
    color: white;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  </style>