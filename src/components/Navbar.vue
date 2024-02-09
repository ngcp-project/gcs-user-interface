<script>
export default {
  data() {
    return {
      isNavbarOpen: false,
      misson_one_status: "Not Started",
      misson_two_status: "Not Started",
      misson_three_status: "Not Started",
    };
  },
  methods: {
    toggleNavbar() {
      this.isNavbarOpen = !this.isNavbarOpen;
    }
  }
};
</script>
<template>
    <nav style="background-color: #011949; padding: 10px;">
      <div style="display: flex; align-items: center;">
        <a href="#" style="text-decoration: none;">
          <img src="../assets/NGCP.svg" alt="" width="40" height="24">
          <span style="font-weight: bold; font-size: 1.2rem; margin-left: 10px;">NG</span>
          <span style="font-weight: bold; font-size: 1.2rem; color: white">CP</span>
        </a>
        <div style="width: 12rem; height:4rem;border: 2px solid rgb(52, 49, 49); background-color: rgb(255, 255, 255); margin-left: auto; display: flex; flex-direction: column; justify-content: center; align-items: flex-start;">
            <div>
                <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0);">Mission 1</span>
                <button style="border: 2px solid rgb(0, 0, 0); margin-left: 1.2rem; color: rgb(0, 0, 0); padding: 3px 6px; font-size: 0.8rem;" type="button" @click="">
                    <span style="font-weight: bold; font-size: 0.8rem;">OPEN</span>
                </button>
            </div>
            <div>
                <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0);">Status: {{ misson_one_status }}</span>
            </div>
        </div>
        <div style="width: 12rem; height:4rem;border: 2px solid rgb(52, 49, 49); background-color: rgb(255, 255, 255); margin-left: auto; display: flex; flex-direction: column; justify-content: center; align-items: flex-start;">
            <div>
                <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0);">Mission 2</span>
                <button style="border: 2px solid rgb(0, 0, 0); margin-left: 1.2rem; color: rgb(0, 0, 0); padding: 3px 6px; font-size: 0.8rem;" type="button" @click="">
                    <span style="font-weight: bold; font-size: 0.8rem;">OPEN</span>
                </button>
            </div>
            <div>
                <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0);">Status: {{ misson_two_status }}</span>
            </div>
        </div>
        <div style="width: 12rem; height:4rem;border: 2px solid rgb(52, 49, 49); background-color: rgb(255, 255, 255); margin-left: auto; display: flex; flex-direction: column; justify-content: center; align-items: flex-start;">
            <div>
                <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0);">Mission 3</span>
                <button style="border: 2px solid rgb(0, 0, 0); margin-left: 1.2rem; color: rgb(0, 0, 0); padding: 3px 6px; font-size: 0.8rem;" type="button" @click="">
                    <span style="font-weight: bold; font-size: 0.8rem;">OPEN</span>
                </button>
            </div>
            <div>
                <span style="font-weight: bold; font-size: 1.2rem; color: rgb(0, 0, 0);">Status: {{ misson_three_status }}</span>
            </div>
        </div>
        <button style="border: 2px solid rgb(255, 0, 0); background-color: rgba(255, 0, 0); margin-left: auto; color: rgb(255, 255, 255);" type="button" @click="">
            <span style="font-size: 18px;">STOP ALL</span>
        </button>
        <button style="border: 2px solid rgb(52, 49, 49); background-color: rgba(38, 36, 36, 0.25); margin-left: auto; color: rgb(52, 49, 49);" type="button" @click="toggleNavbar">
            <span style="font-size: 18px;">&#9776;</span>
        </button>
      </div>
      <div v-if="isNavbarOpen" style="margin-top: 10px;">
        <ul style="list-style-type: none; padding-left: 0;">
          <!-- Insert your menu items here -->
          <li><a href="#" style="text-decoration: none;">Home</a></li>
          <li><a href="#" style="text-decoration: none;">Link</a></li>
          <li><a href="#" style="text-decoration: none;">Disabled</a></li>
        </ul>
      </div>
    </nav>
  </template>
  

  
