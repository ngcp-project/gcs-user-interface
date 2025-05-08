# GCS User Interface

The GCS User Interface for the NGCP project.

# Getting Started

This project uses [Tauri](https://tauri.app/), [Bun](https://bun.sh/), and [Vue 3](https://vuejs.org/).

## Installation

Clone the repository.

```bash
git clone https://github.com/ngcp-project/gcs-user-interface.git
```

Install dependencies.

```bash
bun install
```

Run the application.

```bash
bun tauri dev
```

## Setting up FPV camera server
  
Install Flask and opencv-python dependencies.
  
```bash
pip install flask
```
```bash
pip install opencv-python
```
</details>

Install pyinstaller, this will be used to compile the `.py` file into a binary.

```bash
pip install pyinstaller
```

Create binary from opencv.py

```bash
pyinstaller --onefile opencv.py
```

An `opencv` binary will be created under `dist` folder. Move this into `binaries` folder (if there is already a file, replace it).

Run a Node.js script to rename the binary file, as you must add your architecture to the file name.

```bash
bun run target:triple
```

Now the FPV camera server should run with `bun tauri dev`. It takes time for it to spin up and once it does, make sure you refresh the camera window.
## Setting up map server
Ensure Docker is running then install the docker container.  

```bash
bun run osm:setup
```
 
Run the Map Server container.  

```bash
bun run osm:run
```
</details>
