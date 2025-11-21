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
- NOTE: Ensure PostGreSQL and RabbitMQ images need to be running, along with ONLY the Map server container (bun run osm:run)
- Simply running docker-compose up will run all containers and cause the map server to fail.
```bash
bun tauri
```

## Setting up FPV camera server

Install Flask and opencv-python dependencies.

```bash
pip install flask
```

```bash
pip install opencv-python
```

Install pyinstaller, this will be used to compile the `.py` file into a binary.

```bash
pip install pyinstaller
```

Add binary to Tauri config so it is bundled with the build.  
In `src-tauri/tauri.conf.json`, add `"binaries/opencv"` within the [] of `"externalBin": []`.  

Create binary from opencv.py

```bash
pyinstaller --onefile .\src-tauri\opencv.py --distpath .\src-tauri\binaries\
```

Ensure the`opencv` binary is in the `dist` folder. If needed move this into `binaries` folder (if there is already a file, replace it).

Run a Node.js script to rename the binary file, as you must add your architecture to the file name.

```bash
bun run target:triple
```

Now the FPV camera server should run with `bun tauri dev`. It takes time for it to spin up and once it does, make sure you refresh the camera window.

## Setting up map server

Ensure Docker is running then install the docker container. Note only update

```bash
bun run osm:setup
```

Run the Map Server container.

```bash
bun run osm:run
```

## Running the PostGreSQL Docker image
`docker-compose up db`

## Running the RabbitMQ Docker image
`docker-compose up rabbitmq`

## Map Server Debugging Notes
- If you get an error "Error: role renderer already exists" when running the map server, go into Docker Desktop and delete the volume installed. Re-run the setup command to install the volume again.
