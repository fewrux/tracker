# Tracker

An example RESTful API project built with Actix-Web and Shuttle, showcasing database interaction and basic CRUD operations.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
  - [Configuration](#configuration)
  - [Running the Application](#running-the-application)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Overview

Shuttle API Tracker is a sample Rust project that demonstrates the development of a RESTful API using Actix-Web, backed by a MongoDB database. It includes basic endpoints for adding and retrieving entries. This project serves as a starting point for building more complex Rust-based APIs.

## Features

- RESTful API endpoints for adding and retrieving entries.
- Integration with MongoDB for data storage.
- Actix-Web for HTTP server implementation.

## Getting Started

Follow the instructions below to set up and run the Tracker on your local machine.

### Prerequisites

Before you begin, ensure you have the following prerequisites installed:

- Rust programming language
- Cargo package manager
- MongoDB (local, docker or cloud)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/fewrux/tracker.git
cd tracker
```

2. Build the project:
```bash
cargo build
```

## Usage

To use the Tracker API, follow the steps below.

### Configuration

The application requires a MongoDB URI for database connection. You can set the URI in a `Secrets.toml` file. Example:
```toml
MONGO_URI = "mongodb://localhost:27017/tracker"
```

### Running the Application

To start the application, run the following command:
```bash
cargo shuttle run
```
The Tracker API should be available at http://localhost:8000.

## Testing

You can run unit tests for the application using the following command:
```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Dependencies

```toml
[dependencies]
actix-web = "4.4.0"
anyhow = "1.0.75"
chrono = "0.4.31"
dotenvy = "0.15.7"
env_logger = "0.10.0"
futures = "0.3.28"
log = "0.4.20"
serde = "1.0.188"
shuttle-actix-web = "0.27.0"
shuttle-runtime = "0.27.0"
shuttle-secrets = "0.27.0"
tokio = "1.32.0"

[dependencies.mongodb]
version = "2.6.1"
default-features = false
features = ["async-std-runtime"]
```
