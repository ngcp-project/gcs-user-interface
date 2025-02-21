<script setup lang="ts">
import { Card, CardContent } from "@/components/ui/card";
import {
  Carousel,
  type CarouselApi,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious
} from "@/components/ui/carousel";
import { watchOnce } from "@vueuse/core";
import { ref } from "vue";

import Fade from "embla-carousel-fade";
const emblaMainApi = ref<CarouselApi>();
const emblaThumbnailApi = ref<CarouselApi>();
const selectedIndex = ref(0);

// Hardcoded camera placeholders
const cameras: string[] = ["ERU", "FRA", "MEA", "MRA"];

function onSelect() {
  if (!emblaMainApi.value || !emblaThumbnailApi.value) return;
  selectedIndex.value = emblaMainApi.value.selectedScrollSnap();
  emblaThumbnailApi.value.scrollTo(emblaMainApi.value.selectedScrollSnap());
}

function onThumbClick(index: number) {
  if (!emblaMainApi.value || !emblaThumbnailApi.value) return;
  emblaMainApi.value.scrollTo(index);
}

watchOnce(emblaMainApi, (emblaMainApi) => {
  if (!emblaMainApi) return;

  onSelect();
  emblaMainApi.on("select", onSelect);
  emblaMainApi.on("reInit", onSelect);
});
</script>

<template>
  <!-- Main Carousel -->
  <div class="w-full h-full">
    <!-- Plugin adds fade transition -->
    <Carousel
      class="relative w-full max-w-2xl"
      @init-api="(val) => (emblaMainApi = val)"
      :plugins="[Fade()]"
    >
      <CarouselContent>
        <CarouselItem v-for="(_, index) in 4" :key="index">
          <div class="p-1">
            <Card>
              <CardContent class="flex items-center justify-center p-6">
                <span class="text-4xl font-semibold">{{ cameras[index] }}</span>
                <img v-if="index === 0" src="@/assets/ERU.png" />
                <img v-if="index === 1" src="@/assets/FRA.png" />
                <img v-if="index === 2" src="@/assets/MEA.png" />
                <img v-if="index === 3" src="@/assets/MRA.png" />
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>

    <!-- Carousel Thumbnails -->
    <Carousel class="relative w-full max-w-2xl" @init-api="(val) => (emblaThumbnailApi = val)">
      <CarouselContent class="ml-0 flex justify-center gap-1">
        <CarouselItem
          v-for="(_, index) in 4"
          :key="index"
          class="basis-1/3 cursor-pointer pl-0"
          :class="{ hidden: index === selectedIndex }"
          @click="onThumbClick(index)"
        >
          <div v-if="index !== selectedIndex">
            <div class="p-1" :class="index === selectedIndex ? '' : 'opacity-50'">
              <Card>
                <CardContent class="flex aspect-square items-center justify-center p-6">
                  <span class="text-4xl font-semibold">{{ cameras[index] }}</span>
                  <img v-if="index === 0" src="@/assets/ERU.png" />
                  <img v-if="index === 1" src="@/assets/FRA.png" />
                  <img v-if="index === 2" src="@/assets/MEA.png" />
                  <img v-if="index === 3" src="@/assets/MRA.png" />
                </CardContent>
              </Card>
            </div>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>
  </div>
</template>
