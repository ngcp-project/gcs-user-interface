<script lang="ts">
import { 
    Carousel,
    CarouselContent,
    CarouselItem,
    CarouselNext,
    CarouselPrevious
} from "@/components/ui/carousel";

import "@/assets";

export default {
    
}

</script>

<template>
    <div>
        <Carousel>
            <CarouselContent>
                <CarouselItem>
                    <img
                        src="MEA.png"
                    >
                </CarouselItem>
            </CarouselContent>
        </Carousel>
    </div>
</template>