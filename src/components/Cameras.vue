<script setup lang="ts">
import { Card, CardContent } from "@/components/ui/card";
import {
  Carousel,
  type CarouselApi,
  CarouselContent,
  CarouselItem
} from "@/components/ui/carousel";
import { whenever } from "@vueuse/core";
import { ref } from "vue";

import Fade from "embla-carousel-fade";
const emblaMainApi = ref<CarouselApi>();
const emblaThumbnailApi = ref<CarouselApi>();
const selectedIndex = ref(0);

import { Skeleton } from "@/components/ui/skeleton";

const cameraFeeds = ref([
  { id: 1, name: "UGC", src: "http://127.0.0.1:5000/video_feed" },
  { id: 2, name: "UAV", src: "http://127.0.0.1:5000/video_feed" }
]);

const layout = ref("grid");

// This resets the carousel/emblaAPI everytime we switch layouts, so that the camera switching functions properly.
function cleanupCarousels() {
  if (emblaMainApi.value) {
    emblaMainApi.value.off("select", onSelect);
    emblaMainApi.value.off("reInit", onSelect);
    emblaMainApi.value.destroy();
    emblaMainApi.value = undefined;
  }

  if (emblaThumbnailApi.value) {
    emblaThumbnailApi.value.destroy();
    emblaThumbnailApi.value = undefined;
  }
}

const toggleLayout = () => {
  if (layout.value === "grid") {
    cleanupCarousels();
    layout.value = "carousel";
  } else {
    layout.value = "grid";
  }
};

function onSelect() {
  if (!emblaMainApi.value || !emblaThumbnailApi.value) return;
  selectedIndex.value = emblaMainApi.value.selectedScrollSnap();
  emblaThumbnailApi.value.scrollTo(emblaMainApi.value.selectedScrollSnap());
}

function onThumbClick(index: number) {
  if (!emblaMainApi.value || !emblaThumbnailApi.value) return;
  emblaMainApi.value.scrollTo(index);
}

// This chunk of code gives errors, but it still works properly and is NEEDED to initialize carousel.
whenever(emblaMainApi, (api) => {
  if (!api) return;

  onSelect();
  api.on("select", onSelect);
  api.on("reInit", onSelect);
});
</script>

<template>
  <!-- Side-by-Side (Grid) Layout -->
  <div class="grid-container" v-if="layout === 'grid'">
    <div v-for="feed in cameraFeeds" :key="feed.id">
      <div class="camera">
        <p class="text-center text-xl font-semibold">{{ feed.name }}</p>
        <Card class="cursor-pointer" @click="toggleLayout">
          <CardContent class="flex p-0">
            <Skeleton class="aspect-[4/3] w-full" v-if="!feed.src" />
            <img
              class="image"
              :src="feed.src"
              :alt="feed.name"
              @error="feed.src = ''"
              v-if="feed.src"
            />
          </CardContent>
        </Card>
      </div>
    </div>
  </div>

  <!-- Carousel Layout -->
  <div class="carousel-container" v-else-if="layout === 'carousel'">
    <Carousel class="p-5" @init-api="(val) => (emblaMainApi = val)" :plugins="[Fade()]">
      <CarouselContent>
        <CarouselItem v-for="feed in cameraFeeds" :key="feed.id">
          <div class="focused-camera">
            <p class="text-center text-xl font-semibold">{{ feed.name }}</p>
            <Card class="cursor-pointer" @click="toggleLayout">
              <CardContent class="flex p-0">
                <Skeleton class="aspect-[4/3] w-full" v-if="!feed.src" />
                <img class="image" :src="feed.src" :alt="feed.name" v-if="feed.src" />
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>

    <!-- Carousel Thumbnails -->
    <Carousel
      class="relative max-w-md"
      :opts="{ watchDrag: false }"
      @init-api="(val) => (emblaThumbnailApi = val)"
    >
      <CarouselContent class="flex justify-center gap-5">
        <CarouselItem
          v-for="(feed, index) in cameraFeeds"
          :key="feed.id"
          class="cursor-pointer"
          v-show="index !== selectedIndex"
          @click="onThumbClick(index)"
        >
          <div v-if="index !== selectedIndex">
            <div class="thumbnail-camera">
              <p class="text-center text-xl font-semibold">{{ feed.name }}</p>
              <Card>
                <CardContent class="flex p-0">
                  <Skeleton class="aspect-[4/3] w-full" v-if="!feed.src" />
                  <img class="image" :src="feed.src" :alt="feed.name" v-if="feed.src" />
                </CardContent>
              </Card>
            </div>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>
  </div>
</template>

<style lang="css" scoped>
/* Your existing styles remain the same */
.grid-container {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  height: 90%;
  align-items: center;
  overflow: hidden;
}
.camera {
  width: 45vw;
  margin: auto;
}
.carousel-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  overflow: hidden;
}
.focused-camera {
  width: 45vw;
  margin: auto;
}
.thumbnail-camera {
  width: 10vw;
}
.p-0 {
  padding: 0 !important;
}
.image {
  aspect-ratio: 4/3;
  border-radius: 0.5rem;
  object-fit: contain;
  width: 100%;
}
</style>
