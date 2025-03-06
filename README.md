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

## Running the Map Server

Ensure Docker is running then install the docker container.

```bash
bun run osm:setup
```

Run the Map Server container.

```bash
bun run osm:run
```
