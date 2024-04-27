<template>
  <transition name="modal-fade">
    <div class="modal-backdrop" @click="closeFromOutside">
      <div class="modal">
        <header class="modal-header">
          <img src="../../assets/stop-symbol.png" class="stop-icon"/> 
          <slot name="header">
            Emergency Stop
          </slot>
          <button type="button" class="btn-close" @click="close">x</button>
        </header>
        
        <section class="modal-body">
          <slot name="body">
            Send Stop Command to {{ vehicleName }}?
          </slot>
         </section>
  
        <footer class="modal-footer">
          <button type="button" class="modal-button no-button" @click="close"> No </button>
          <button type="button" class="modal-button yes-button" @click="sendStopCommand"> Yes </button>
        </footer>
      </div>
    </div>
  </transition>
</template>
    
<script lang="ts">
import axios, {AxiosResponse} from "axios";
    export default {
        data() {
            return {};    
        },
        props: {
            vehicleName: { required: true, type: String},
        },
        methods: {
          sendStopCommand() {
            // console.log("Pressed yes to send stop command for " + this.vehicleName);
            // //console.error({ key: this.vehicleName });
            // axios.post('http://localhost:5135/EmergencyStop', { key: this.vehicleName })
            //     .then((response: AxiosResponse<any>) => {
            //         console.log('Stop command sent successfully:', response.data);
            //     })
            //     .catch( (error: any) => {
            //         console.error('Error sending stop command:', error);
            //     });
            // this.close(); // Close the dialog or modal after sending the command
            console.error(JSON.stringify({ Key: this.vehicleName }));
            fetch('http://localhost:5135/EmergencyStop', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ Key: this.vehicleName })
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then(data => console.log(data))
            .catch(error => console.error('Error sending stop command:', error));
          },
            close() {
                this.$emit('close');
            },
            closeFromOutside(event: MouseEvent) {
                if (!this.$el.querySelector('.modal').contains(event.target)) {
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
    background: #FFFFFF;
    box-shadow: 2px 2px 20px 1px;
    overflow-x: auto;
    display: flex;
    flex-direction: column;
    width: 25%;
    height: 22%;
    border-radius: 2%;
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
    font-size: 1.8em;
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
    transition: color 0.20s;
  }

  .btn-close:hover {
    color: rgb(163, 9, 9);
  }

  .modal-button {
    display:flex;
    justify-content: center;
    color: white;
    width: 15%;
    border-radius: 10%;
    transition: background-color 0.30s;
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
    transition: opacity .3s ease;
  }

/* slide transitions for modal window */
  .modal-fade-enter-from .modal,
  .modal-fade-leave-to .modal {
    transform: translateY(-40px);
    transition: transform .3s ease;
  }

  .stop-icon {
    width: 5%;
  }
</style>    