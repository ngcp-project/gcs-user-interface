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

## Running the Map Server

Ensure Docker is running then install the docker container.

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