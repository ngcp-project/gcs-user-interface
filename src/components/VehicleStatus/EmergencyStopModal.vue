<template>
  <transition name="modal-fade">
    <div class="modal-backdrop" @click="closeFromOutside">
      <div class="modal">
        <header class="modal-header">
          <img src="../../assets/stop-symbol.png" class="stop-icon" />
          <slot name="header"> Emergency Stop </slot>
          <button type="button" class="btn-close" @click="close">x</button>
        </header>

        <section class="modal-body">
          <slot name="body">
            <div v-if="vehicleName == 'all'" style="font-size: 1.6em">
              Send Stop Command to All Vehicles?
            </div>
            <div v-else class="bar" style="font-size: 1.8em">
              Send Stop Command to {{ vehicleName }}?
            </div>
          </slot>
        </section>

        <footer class="modal-footer">
          <button type="button" class="modal-button no-button" @click="close">No</button>
          <button type="button" class="modal-button yes-button" @click="sendStopCommand">
            Yes
          </button>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script lang="ts">
export default {
  data() {
    return {
      vehicle_names: ["ERU", "MEA", "MRA", "FRA"]
    };
  },
  props: {
    vehicleName: { required: true, type: String }
  },
  methods: {
    sendStopCommand() {
      const promises: any[] = [];
      if (this.vehicleName == "all") {
        // send Emergency Stop command for all vehicles
        this.vehicle_names.forEach((name) => {
          const promise = fetch("http://localhost:5135/EmergencyStop", {
            method: "POST",
            headers: {
              "Content-Type": "application/json"
            },
            body: JSON.stringify({ Key: name })
          })
            .then((response) => {
              if (!response.ok) {
                throw new Error("Network response was not ok");
              }
              return response.json();
            })
            .then((data) => console.log(data))
            .catch((error) => console.error("Error sending stop command:", error));
          promises.push(promise);
        });

        Promise.all(promises)
          .then((data) => {
            console.log("Sent stop commands to all four vehicles!");
          })
          .catch((error) => {
            console.error("Error sending stop command:", error);
          });
      } else {
        // send Emergency Stop command for specific vehicle
        fetch("http://localhost:5135/EmergencyStop", {
          method: "POST",
          headers: {
            "Content-Type": "application/json"
          },
          body: JSON.stringify({ Key: this.vehicleName })
        })
          .then((response) => {
            if (!response.ok) {
              throw new Error("Network response was not ok");
            }
            return response.json();
          })
          .then((data) => console.log(data))
          .catch((error) => console.error("Error sending stop command:", error));
      } //end else
      this.close(); // close modal
    },

    close() {
      this.$emit("close"); // emit event called 'close' to parent component (EmergencyStop.vue) to tell it to close the modal
    },

    closeFromOutside(event: MouseEvent) {
      if (!this.$el.querySelector(".modal").contains(event.target)) {
        this.close();
      }
    }
  }
};
</script>

<style scoped>
.modal-backdrop {
  position: fixed;
  display: flex;
  justify-content: center;
  align-items: center;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: rgba(0, 0, 0, 0.3);
  z-index: 999;
}

.modal {
  background: #ffffff;
  box-shadow: 2px 2px 20px 1px;
  overflow-x: auto;
  display: flex;
  flex-direction: column;
  width: 25%;
  height: 22%;
  border-radius: 2%;
  color: black;
}

.modal-header {
  position: relative;
  display: flex;
  /* justify-content: space-between; */
  gap: 2%;
  /* padding: 15px; */
  padding: 15px;

  font-size: 1.2em;
  font-weight: 600;
  border-bottom: 1px solid #d1cece;
}

.modal-body {
  position: relative;
  padding: 20px 10px;
  font-weight: bold;
}

.modal-footer {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 20%;
  padding-top: 5%;
}

.btn-close {
  position: absolute;
  top: 2%;
  right: 2%;
  border: none;
  font-size: 20px;
  padding: 10px;
  cursor: pointer;
  font-weight: bold;
  color: rgb(255, 57, 57);
  background: transparent;
  transition: color 0.2s;
}

.btn-close:hover {
  color: rgb(163, 9, 9);
}

.modal-button {
  display: flex;
  justify-content: center;
  color: white;
  width: 15%;
  border-radius: 10%;
  transition: background-color 0.3s;
  font-size: 1.2em;
}

.no-button {
  background: rgb(255, 57, 57);
  border: 1px solid rgb(177, 6, 6);
}
.no-button:hover {
  background-color: rgb(192, 40, 40);
}

.yes-button {
  background: #03b900;
  border: 1px solid #059402;
}
.yes-button:hover {
  background-color: #027500;
}

/* fade in/out transitions for the modal + modal overlay (screen dimming)*/
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

/* slide transitions for modal window */
.modal-fade-enter-from .modal,
.modal-fade-leave-to .modal {
  transform: translateY(-40px);
  transition: transform 0.3s ease;
}

.stop-icon {
  width: 5%;
}
</style>
