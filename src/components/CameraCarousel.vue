<script setup lang="ts">
import { Card, CardContent } from "@/components/ui/card";
import {
  Carousel,
  type CarouselApi,
  CarouselContent,
  CarouselItem
} from "@/components/ui/carousel";
import { watchOnce } from "@vueuse/core";
import { ref } from "vue";

import Fade from "embla-carousel-fade";
const emblaMainApi = ref<CarouselApi>();
const emblaThumbnailApi = ref<CarouselApi>();
const selectedIndex = ref(0);

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
  <div class="carousel-container">
    <!-- Plugin adds fade transition -->
    <Carousel
      class="overflow-hidden p-5"
      @init-api="(val) => (emblaMainApi = val)"
      :plugins="[Fade()]"
    >
      <CarouselContent>
        <CarouselItem v-for="(_, index) in 3" :key="index">
          <div class="focused-camera">
            <Card>
              <CardContent class="flex p-0">
                <!-- <span class="text-4xl font-semibold">{{ cameras[index] }}</span> -->
                <img class="image" v-if="index === 0" src="@/assets/ERU-view.jpg" />
                <img class="image" v-if="index === 1" src="@/assets/MEA-view.jpg" />
                <img class="image" v-if="index === 2" src="@/assets/MRA-view.jpg" />
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>

    <!-- Carousel Thumbnails -->
    <Carousel
      class="relative max-w-lg items-center justify-center"
      @init-api="(val) => (emblaThumbnailApi = val)"
    >
      <CarouselContent class="flex justify-center gap-5">
        <CarouselItem
          v-for="(_, index) in 3"
          :key="index"
          class="basis-1/2 cursor-pointer pl-0"
          :class="{ hidden: index === selectedIndex }"
          @click="onThumbClick(index)"
        >
          <!-- NOTE: You can click and drag around the thumbnails. This is meant for multiple thumbnails that don't fit the carousel. I do not know how to disable this feature.-->
          <div v-if="index !== selectedIndex">
            <Card>
              <CardContent class="flex p-0">
                <!-- <span class="text-4xl font-semibold">{{ cameras[index] }}</span> -->
                <img class="image" v-if="index === 0" src="@/assets/ERU-view.jpg" />
                <img class="image" v-if="index === 1" src="@/assets/MEA-view.jpg" />
                <img class="image" v-if="index === 2" src="@/assets/MRA-view.jpg" />
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>
  </div>
</template>

<style lang="css" scoped>
.carousel-container {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.focused-camera {
  max-height: 70vh;
  max-width: 60vw;
  margin: auto;
  display: flex;
  justify-content: center;
  align-items: center;
}
.p-0 {
  /* Removes card padding inherited from CardContent UI */
  padding: 0 !important;
}
.image {
  aspect-ratio: 5/3;
  border-radius: 0.5rem;
  object-fit: fill;
  width: 100%;
  height: 100%;
}
</style>
