<script lang="ts">
import MissionStatus from "./MissionStage/MissionStatus.vue";
import EmergencyStopModal from "../components/VehicleStatus/EmergencyStopModal.vue";
import ThemeToggle from "./ThemeToggle.vue";
import { NgButton } from "./ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import { Icon } from "@iconify/vue";

export default {
  components: {
    EmergencyStopModal,
    MissionStatus,
    ThemeToggle,
    // eslint-disable-next-line vue/no-reserved-component-names
    NgButton,
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
    Icon
  },
  data() {
    return {
      isNavbarOpen: false,
      misson_one_status: "Done",
      misson_two_status: "In Progress",
      misson_three_status: "initiated",
      // track whether to display EmergencyStopModal
      showModal: false,
      // send to EmergencyStopModal to indicate we want to stop all vehicles
      stop_all: "all",
      // nav links
      nav_links: [
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
      ]
    };
  },
  methods: {
    toggleNavbar() {
      this.isNavbarOpen = !this.isNavbarOpen;
    },
    showEmergencyModal() {
      this.showModal = true;
    },
    closeEmergencyModal() {
      this.showModal = false;
    }
  }
};
</script>

<template>
  <nav class="h-[88px] bg-background p-3">
    <div style="display: flex; align-items: center; gap: 5%">
      <router-link to="/" class="text-lg font-bold text-[#249b73]">
        <span class="hover:text-[#646cff]">NG</span>CP
      </router-link>
      <MissionStatus :missionNumber="1" :status="misson_one_status" />
      <!-- <MissionStatus :missionNumber="2" :status="misson_two_status" />
      <MissionStatus :missionNumber="3" :status="misson_three_status" /> -->
      <!-- <button
        style="border: 2px solid rgb(255, 0, 0); background-color: rgba(23, 0, 0); color: rgb(255, 255, 255)" type="button" @click="refresh_MISSION()">
        <span style="font-size: 18px">Refresh Mission</span>
      </button> -->
      <NgButton @click="showEmergencyModal" class="text-lg font-bold" variant="destructive">
        STOP ALL
      </NgButton>
      <EmergencyStopModal
        :vehicle-name="stop_all"
        v-show="showModal"
        @close="closeEmergencyModal"
      ></EmergencyStopModal>
      <div class="flex items-center gap-2">
        <ThemeToggle />
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <NgButton @click="toggleNavbar" size="icon" variant="secondary">
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
