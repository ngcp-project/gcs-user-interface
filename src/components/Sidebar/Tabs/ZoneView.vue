<script setup lang="ts">
import ZoneCard from '@/components/SidebarCards/ZoneCard.vue';
import SidebarHeader from '@/components/ui/sidebar/SidebarHeader.vue';
</script>

<template>
  <div>
    <SidebarGroup>
      <ZoneCard zoneType="In" />
      <ZoneCard zoneType="Out" />
    </SidebarGroup>
  </div>
</template>