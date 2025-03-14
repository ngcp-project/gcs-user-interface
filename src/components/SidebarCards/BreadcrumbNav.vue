<script setup lang="ts">
import { Breadcrumb, BreadcrumbItem, BreadcrumbSeparator } from '@/components/ui/breadcrumb';

defineProps<{
  currentState: string;
  navigateTo: (view: string) => void;
}>();
</script>

<template>
  <Breadcrumb>
    <BreadcrumbItem 
      :active="currentState === 'mission'" 
      class="cursor-pointer text-secondary" 
      @click="navigateTo('mission')"
    >
      Mission
    </BreadcrumbItem>
    <BreadcrumbSeparator class="text-secondary" />
    <BreadcrumbItem 
      :active="currentState === 'vehicle'" 
      class="cursor-pointer text-secondary" 
      @click="navigateTo('vehicle')"
    >
      Vehicle
    </BreadcrumbItem>
    <BreadcrumbSeparator class="text-secondary" />
    <BreadcrumbItem 
      :active="currentState === 'stage'" 
      class="cursor-pointer text-secondary" 
      @click="navigateTo('stage')"
    >
      Stage
    </BreadcrumbItem>
  </Breadcrumb>
</template>
