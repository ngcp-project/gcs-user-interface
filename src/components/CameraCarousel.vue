<script lang="ts">
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious
} from "@/components/ui/carousel";

export default {};
</script>

<template>
  <Carousel class="relative w-full max-w-xs">
    <CarouselContent class="flex">
      <CarouselItem class="basis-full">
        <img
          src="../assets/MRA.png"
          alt="Carousel Image 1"
          class="w-full h-[400px] object-cover rounded-lg"
        />
      </CarouselItem>
      <CarouselItem class="basis-full">
        <img
          src="../assets/MRA.png"
          alt="Carousel Image 2"
          class="w-full h-[400px] object-cover rounded-lg"
        />
      </CarouselItem>
      <CarouselItem class="basis-full">
        <img
          src="../assets/MRA.png"
          alt="Carousel Image 3"
          class="w-full h-[400px] object-cover rounded-lg"
        />
      </CarouselItem>
      <CarouselItem class="basis-full">
        <img
          src="../assets/MRA.png"
          alt="Carousel Image 4"
          class="w-full h-[400px] object-cover rounded-lg"
        />
      </CarouselItem>
      <CarouselItem class="basis-full">
        <img
          src="../assets/MRA.png"
          alt="Carousel Image 5"
          class="w-full h-[400px] object-cover rounded-lg"
        />
      </CarouselItem>
    </CarouselContent>
    <CarouselPrevious />
    <CarouselNext />
  </Carousel>
</template>
