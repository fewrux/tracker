# Tracker

Tracker is a tool designed to monitor device usage by logging time entries while the device is online. The application consists of a Web Server and RESTful API, both powered by Actix-Web and Shuttle technologies. The entries are stored in a MongoDB Cloud Cluster for reliability and scalability.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Getting Started](#getting-started)
  - [Windows](#windows)
  - [Linux](#linux)
- [Environment Configuration](#environment-configuration)
  - [Cloud](#cloud)
    - [Configuration](#configuration)
    - [Checking the Logs](#checking-the-logs)
  - [Local](#local)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Database Connection](#database-connection)
    - [Running the Application](#running-the-application)
    - [Usage](#usage)
- [Testing](#testing)
- [License](#license)

## Overview

Shuttle API Tracker is a sample Rust project that demonstrates the development of a RESTful API using Actix-Web, backed by a MongoDB database. It includes basic endpoints for adding and retrieving entries. This project serves as a starting point for building more complex Rust-based APIs.

## Features

- RESTful API endpoints for adding and retrieving entries.
- Integration with MongoDB for data storage.
- Actix-Web for HTTP server implementation.

## Getting Started

Get started with Tracker by setting up a background script that logs your system's uptime by polling the Tracker API at regular intervals. Follow the instructions below to configure and run the Tracker script on your device.

### Windows

To ensure that the Tracker script runs on Windows startup, you can use a combination of a batch file (`tracker.bat`) and a VBScript (`launch_tracker.vbs`). Follow these steps:

**Step 1: Create the Batch File (`tracker.bat`)**

1. Create a batch file named `tracker.bat` (you can use any text editor, e.g., Notepad).

2. Add the following content to `tracker.bat`:

   ```batch
   @echo off
   :loop
   curl <API_URL>
   timeout /t 60
   goto loop
   ```
   Replace `<API_URL>` with your local or cloud URL.
   This batch file contains a loop that makes a curl request to your desired URL and waits for 60 seconds between each request.

**Step 2: Create the VBScript (`launch_tracker.vbs`)**

1. Create a VBScript named launch_tracker.vbs (you can use any text editor, e.g., Notepad).

2. Add the following content to launch_tracker.vbs:
   ```vbscript
   Set WshShell = CreateObject("WScript.Shell")
   WshShell.Run chr(34) & "C:\path\to\tracker.bat" & Chr(34), 0
   Set WshShell = Nothing
   ```
   Replace `C:\path\to\tracker.bat` with the actual path to your `tracker.bat` file.

**Step 3: Configure Windows Startup**

1. Press `Win + R` to open the Run dialog.
   
2. Type `shell:startup` and press Enter. This will open the Startup folder.
   
3. Copy both the `tracker.bat` and `launch_tracker.vbs` files into the Startup folder.

**Step 4: Verify Configuration**

1. Restart your Windows computer.

2. Press Ctrl + Shift + Esc or Ctrl + Alt + Delete, then select "Task Manager" from the menu that appears.

3. In the Task Manager window, navigate to the "Processes" or "Details" tab (depending on your Windows version).

4. Look for a process named tracker.bat or cmd.exe (if tracker.bat is running in a Command Prompt window).

5. That's it! Now you can customize the script and adjust the startup behavior as needed.

##

### Linux

To configure the Tracker script on a Linux system to run at startup, you can use a combination of a shell script (`tracker.sh`) and a systemd service (`launch_tracker.service`). Follow these steps:

**Step 1: Create the Shell Script (`tracker.sh`)**

1. Create a shell script named `tracker.sh` (you can use any text editor, e.g., Nano or Vim).

2. Add the following content to `tracker.sh`:

   ```bash
   #!/bin/bash
   while true; do
     curl <API_URL>
     sleep 60
   done
   ```
   Replace `<API_URL>` with your local or cloud URL.
   This shell script contains a loop that makes a curl request to your desired URL and waits for 60 seconds between each request.

**Step 2: Create the systemd Service (`launch_tracker.service`)**

1. Create a systemd service unit file named `launch_tracker.service`, under `/etc/systemd/system`.

2. Add the following content to `launch_tracker.service`:
   ```bash
   [Unit]
   Description=Tracker Startup Script.

   [Service]
   Type=simple
   RemainAfterExit=yes
   Restart=always
   ExecStart=/bin/bash /path/to/tracker.sh

   [Install]
   WantedBy=multi-user.target
   ```
   Replace `/path/to/tracker.sh` with the actual path to your `tracker.sh` script.

**Step 3: Set file permission and enable service**

1. On your terminal, set the file permissions to 644:
   ```bash
   chmod 644 /etc/systemd/system/launch_tracker.service
   ```
2. Enable the service to run at startup:
   ```bash
   systemctl enable launch_tracker.service
   ```

**Step 4: Verify Configuration**

1. Restart your Linux computer.
   
2. Check the status of the service using:
   ```bash
   sudo systemctl status launch_tracker.service
   ```
3. Customize the script and adjust the startup behavior as needed.

## Environment Configuration

### Cloud

**Note:** To use the Cloud configuration, a stable internet connection is a prerequisite. If you need to monitor system uptime offline, please refer to the [Local](#local) section for the appropriate setup.

#### Configuration

1. Open your Tracker script and replace `<API_URL>` with the correct endpoint.
  - Windows - `tracker.bat`:
     ```batch
     @echo off
     :loop
     curl https://example.com/api/endpoint
     timeout /t 60
     goto loop
     ```
   
  - Linux - `tracker.sh`:
    ```bash
    #!binbash
    while true; do
      curl https://tracker.shuttleapp.rs/entries/add
      sleep 60
    done
    ```

2. Save the file and restart your computer.

#### Checking the Logs

To access the application logs and monitor system uptime, you can easily do so by accessing the `<API_URL>` in your internet browser. This will provide you with real-time access to the system's usage logs.

##

### Local

#### Prerequisites

Before you start using Tracker locally, make sure you have the following prerequisites installed:

- [Rust programming language](https://www.rust-lang.org/)
- [Cargo package manager](https://doc.rust-lang.org/cargo/)
- [MongoDB](https://www.mongodb.com/) (local, Docker, or cloud)

#### Installation

1. Clone the repository:
  ```bash
  git clone https://github.com/fewrux/tracker.git
  cd tracker
  ```

2. Build the project:
  ```bash
  cargo build
  ```

#### Database Connection

The application requires a MongoDB URI for database connection. You can set the URI in a `Secrets.toml` file. Example:
```toml
MONGO_URI = "mongodb://localhost:27017/tracker"
```

#### Running the Application

To start the application, run the following command:
```bash
cargo shuttle run
```
The Tracker API should be available at http://localhost:8000.

#### Usage

To access the application logs and monitor system uptime, follow these steps:

1. Open a terminal window.
2. Navigate to the project directory.
3. Run the following command to view the logs:
   ```bash
   cargo shuttle logs
   ```

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
