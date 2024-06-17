<script setup lang="ts">
import MissionStatus from "./MissionStage/MissionStatus.vue";
import EmergencyStopModal from "../components/VehicleStatus/EmergencyStopDialog.vue";
import ThemeToggle from "./ThemeToggle.vue";
import { NgButton } from "./ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import { Icon } from "@iconify/vue";
import { ref } from "vue";

const misson_one_status = ref("Done");
const misson_two_status = ref("In Progress");
const misson_three_status = ref("initiated");

// nav links
const nav_links = [
  {
    label: "Home",
    href: "/"
  },
  {
    label: "Map Screen",
    href: "/StaticScreen"
  },
  {
    label: "Mission Initialization",
    href: "/MissionInitialization"
  },
  {
    label: "Test",
    href: "/test"
  }
];
</script>

<template>
  <nav class="h-fit bg-background p-3">
    <div class="flex items-center justify-between gap-2">
      <router-link to="/" class="text-3xl font-bold tracking-wider text-[#249b73]">
        <span class="hover:text-[#646cff]">NG</span>CP
      </router-link>
      <div class="flex items-center gap-2">
        <MissionStatus :missionNumber="1" :status="misson_one_status" />
        <!-- <MissionStatus :missionNumber="2" :status="misson_two_status" />
        <MissionStatus :missionNumber="3" :status="misson_three_status" /> -->
        <!-- <button
          style="border: 2px solid rgb(255, 0, 0); background-color: rgba(23, 0, 0); color: rgb(255, 255, 255)" type="button" @click="refresh_MISSION()">
          <span style="font-size: 18px">Refresh Mission</span>
        </button> -->
        <!-- <NgButton @click="showEmergencyModal" variant="destructive"> STOP ALL </NgButton> -->
        <EmergencyStopModal :vehicle-name="'all'" />
      </div>

      <div class="flex items-center gap-2">
        <ThemeToggle />
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <NgButton size="icon" variant="secondary">
              <Icon icon="radix-icons:hamburger-menu" />
            </NgButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <router-link v-for="(nav, index) in nav_links" :key="index" :to="nav.href">
              <DropdownMenuItem class="cursor-pointer">{{ nav.label }}</DropdownMenuItem>
            </router-link>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </div>
  </nav>
</template>
