<script setup lang="ts">
import {
  Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader,
} from '@/components/ui/sidebar'
import {Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList,
  BreadcrumbPage, BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
</script>

<template>
  <Sidebar>
    <SidebarHeader>
      <Breadcrumb>
      </Breadcrumb>
    </SidebarHeader>
    <SidebarContent>

      <MISSIONVIEW if=(state==mission)> // import this from another file
        <SidebarGroup>
          CARDS...
          <ADDBUTTON css=(bottom of page)/>
        <SidebarGroup/>
      <MISSIONVIEW/>
      
      <VEHICLEVIEW if=(state==vehicle)> // import this from another file
        <SidebarGroup>
          CARDS...
          <ADDBUTTON css=(bottom of page)/>
        <SidebarGroup/>
      <VEHICLEVIEW/>

      <STAGEVIEW if=(state==stage)> // import this from another file
        <SidebarGroup>
          CARDS...
          <ADDBUTTON css=(bottom of page)/>
        <SidebarGroup/>
      <STAGEVIEW/>

    </SidebarContent>
  </Sidebar>
</template>