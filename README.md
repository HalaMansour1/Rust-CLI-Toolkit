# Rust CLI Tool-Kit

![Project Logo](./assets/rusty.png)

## Overview

The Rust CLI Tool-Kit is a versatile command line toolkit designed to provide a range of utilities for developers and users. It includes tools for managing to-do lists, taking notes, integrating calendars, and color-coordinating errors for easier debugging.

## Features

- **To-Do List Manager:** Create, edit, delete, and list tasks with a simple CLI interface.
- **Note-Taking Application:** Save and retrieve notes directly from the command line.
- **Calendar Integration:** Display a calendar view and manage events.
- **Color-Error Coordinator:** Enhances your interface by color-coding errors and providing explanations.

## Installation

To install the Rust CLI Tool-Kit, follow these steps:

1. **Clone the Repository:**

    ```sh
    git clone https://github.com/HalaMansour1/rust-cli-toolkit.git
    ```

2. **Navigate to the Project Directory:**

    ```sh
    cd rust-cli-toolkit
    ```

3. **Build the Project:**

    ```sh
    cargo build --release
    ```

4. **Run the Project:**

    ```sh
    cargo run
    ```

## Usage

### To-Do List Manager

#### Add a Task

```sh
cargo run add "Task name"

