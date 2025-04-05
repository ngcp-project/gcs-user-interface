<script setup lang="ts">
import ZoneCard from '@/components/Sidebar/SidebarCards/ZoneCard.vue';
import SidebarGroup from '@/components/ui/sidebar/SidebarGroup.vue';
</script>

<template>
  <div>
    <SidebarGroup>
      <ZoneCard zoneType="KeepIn" />
      <ZoneCard zoneType="KeepOut" />
    </SidebarGroup>
  </div>
</template>
