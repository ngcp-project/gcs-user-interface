<script setup lang="ts">
import { Card, CardContent } from '@/components/ui/card'
import { Carousel, type CarouselApi, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from '@/components/ui/carousel'
import { watchOnce } from '@vueuse/core'
import { ref } from 'vue'

import Fade from 'embla-carousel-fade'
const emblaMainApi = ref<CarouselApi>()
const emblaThumbnailApi = ref<CarouselApi>()
const selectedIndex = ref(0)

function onSelect() {
  if (!emblaMainApi.value || !emblaThumbnailApi.value)
    return
  selectedIndex.value = emblaMainApi.value.selectedScrollSnap()
  emblaThumbnailApi.value.scrollTo(emblaMainApi.value.selectedScrollSnap())
}

function onThumbClick(index: number) {
  if (!emblaMainApi.value || !emblaThumbnailApi.value)
    return
  emblaMainApi.value.scrollTo(index)
}

watchOnce(emblaMainApi, (emblaMainApi) => {
  if (!emblaMainApi)
    return

  onSelect()
  emblaMainApi.on('select', onSelect)
  emblaMainApi.on('reInit', onSelect)
})
</script>

<template>
  <div class="w-full sm:w-auto">
    <!-- Plugin adds fade transition -->
    <Carousel
      class="relative w-full max-w-xs"
      @init-api="(val) => emblaMainApi = val"
      :plugins="[Fade()]" 
    >
      <CarouselContent>
        <CarouselItem v-for="(_, index) in 3" :key="index">
          <div class="p-1">
            <Card>
              <CardContent class="flex items-center justify-center p-6">
                <span class="text-4xl font-semibold">{{ index + 1 }}</span>
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>

    <Carousel
      class="relative w-full max-w-xs"
      @init-api="(val) => emblaThumbnailApi = val"
    >
      <CarouselContent class="flex gap-1 ml-0 justify-center">
        <CarouselItem v-for="(_, index) in 3" :key="index" class="pl-0 basis-1/4 cursor-pointer" @click="onThumbClick(index)">
          <div class="p-1" :class="index === selectedIndex ? '' : 'opacity-50'">
            <Card>
              <CardContent class="flex aspect-square items-center justify-center p-6">
                <span class="text-4xl font-semibold">{{ index + 1 }}</span>
              </CardContent>
            </Card>
          </div>
        </CarouselItem>
      </CarouselContent>
    </Carousel>
  </div>
</template>