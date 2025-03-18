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

// NOTE: To run the cameras for development, run the flask server from https://github.com/ngcp-project/gcs-infrastructure-fpvtest/tree/josh
const cameraFeeds = ref([
  { id: 1, name: "UGC", src: "http://127.0.0.1:5000/video_feed" },
  { id: 2, name: "UAV", src: "http://127.0.0.1:5000/video_feed" }
]);

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
        <CarouselItem v-for="(feed, index) in cameraFeeds" :key="feed.id">
          <div class="focused-camera">
            <Card>
              <CardContent class="flex p-0">
                <img class="image" :src="feed.src" :alt="feed.name" />
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
          v-for="(feed, index) in cameraFeeds"
          :key="feed.id"
          class="basis-1/2 cursor-pointer pl-0"
          v-show="index !== selectedIndex"
          @click="onThumbClick(index)"
        >
          <!-- NOTE: You can click and drag around the thumbnails. This is meant for multiple thumbnails that don't fit the carousel. I do not know how to disable this feature.-->
          <div v-if="index !== selectedIndex">
            <Card>
              <CardContent class="flex p-0">
                <!-- <span class="text-4xl font-semibold">{{ cameras[index] }}</span> -->
                <img class="image" :src="feed.src" :alt="feed.name" />
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
