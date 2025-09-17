# Rust Slint Task Manager


A desktop companion for software projects built with [Slint](https://slint.dev/) and Rust. The app keeps your backlog close at hand with a polished native UI, rich metadata for every task, and a lightweight focus log so you can see what you are working on.


<div align="center">
  <video src="demo.webm" controls style="max-width: 100%; border-radius: 12px;">
    Your browser does not support the video tag.
  </video>
</div>


## Overview

- **Purpose-built for developers** – Track tracker links, review URLs, branch names, commit templates, and notes without reaching for a spreadsheet.
- **Fast native experience** – Slint renders a fluid interface that feels at home on Windows, macOS, and Linux.
- **Local-first storage** – Data lives in a bundled SQLite database so your tasks persist without setting up a backend service.

## Feature Highlights

- **Organised task list** – Toggle sorting between title and creation time while keeping the current selection steady.
- **Detailed editor** – Update metadata inline and see when the task was created in your local timezone.
- **Current focus tracking** – Mark a task as "current"; every change appends a row to a work log table for future analytics.
- **Guided empty state** – The first run seeds a sample task so you can explore the interface immediately.
- **About dialog** – Access a quick app summary from the header for sharing with teammates.

## UI Tour

### 1. Header bar
A compact banner surfaces the app name, sort toggles, and quick access to the About dialog.

### 2. Task list panel
Scroll through saved tasks, highlight the active one, and instantly switch between alphabetical or chronological order. Tasks marked as current display a badge to keep them visible.

### 3. Task creation row
Create new items from the detail side with a single text field and keyboard-friendly button.

### 4. Detail editor
Review and edit the selected task's metadata, including tracker links, code branches, review URLs, commit message templates, and free-form notes. The panel shows loading states while data is fetched on a background thread.

### 5. Focus controls
Use the "Set as current" button to log the start time of your focus session in the `work_log` table.

## Data Model

The app creates and maintains three SQLite tables the first time it runs:

| Table | Purpose |
| ----- | ------- |
| `tasks` | Stores task metadata (title, timestamps, links, notes, and whether the task is current). |
| `settings` | Persists the selected sort order across launches. |
| `work_log` | Records a timestamp whenever a task is marked current to support future reporting. |

`tasks.db` is placed beside the executable. Remove the file to reset the application state.


## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or newer

- No extra system dependencies are required because `rusqlite` is compiled with bundled SQLite.

### Run the application

```bash
cargo run
```


A sample task is inserted on first launch so you can try the workflow immediately.

### Build an optimised binary


```bash
cargo build --release
```


The optimised executable is available at `target/release/rust-slint` (or with the appropriate platform extension).

## Development Workflow

- `cargo fmt` – Format the Rust sources.
- `cargo clippy --all-targets --all-features` – Lint the project for common pitfalls.
- `cargo check` – Type-check the codebase quickly while iterating.
- (Planned) Add unit and integration tests as outlined in the [roadmap](ROADMAP.md).

## Project Layout

```
.
├── src/main.rs        # Application logic, database interactions, Slint callbacks
├── ui/main.slint      # Declarative UI layout and styling
├── assets/            # Static imagery used in the UI (logo, etc.)
├── build.rs           # Builds the Slint UI module before compilation
├── demo.webm          # Short screencast of the main workflow
└── Cargo.toml         # Rust workspace manifest with dependencies
```

## Roadmap

Curious about what is next? Dive into [ROADMAP.md](ROADMAP.md) for a staged improvement plan covering polish, productivity features, and long-term ambitions.

## License

Distributed under the terms of the [MIT License](LICENSE).

