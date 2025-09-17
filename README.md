# Rust Slint Task Manager

A desktop task tracker built with [Slint](https://slint.dev/) and Rust. The app focuses on keeping track of software development work with fields for tracker links, review URLs, commit message templates, and rich notes. Data is stored locally in an embedded SQLite database so your tasks persist between runs.

<div align="center">
  <video src="demo.webm" controls style="max-width: 100%; border-radius: 12px;">
    Your browser does not support the video tag.
  </video>
</div>

## Features

- **Task list with quick filters** &mdash; Toggle between sorting by title or creation time while keeping the current selection intact.
- **Detailed task editor** &mdash; Edit metadata such as tracker URL, branch name, review URL, commit template, and free-form notes.
- **Current task tracking** &mdash; Mark a task as your active focus. The app logs the timestamp in the `work_log` table for lightweight time tracking.
- **Persistent storage** &mdash; Tasks are stored in a local `tasks.db` SQLite database so they are available on every launch.
- **Responsive UI** &mdash; Loading indicator and background thread fetching keep the interface responsive when switching between tasks.
- **Cross-platform** &mdash; Powered by Slint's winit backend and femtovg renderer for Windows, macOS, and Linux support.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or newer
- No additional system dependencies are required thanks to the bundled SQLite feature enabled for `rusqlite`.

### Run the application

```bash
cargo run
```

The first launch creates a sample task in `tasks.db`. Subsequent runs reuse the same database, so your work persists. Delete `tasks.db` if you want to start from a clean slate.

### Build a release binary

```bash
cargo build --release
```

The optimized executable will be written to `target/release/rust-slint`.

## Project Structure

- `src/main.rs` &mdash; Application logic, database access, and wiring between Rust and Slint.
- `ui/main.slint` &mdash; Declarative UI describing the windows, layouts, and widgets.
- `assets/` &mdash; Static resources such as the Slint logo used in the About dialog.
- `demo.webm` &mdash; Short screencast showcasing the main workflow.

## Database Schema

The bundled SQLite database automatically creates three tables:

| Table | Purpose |
| ----- | ------- |
| `tasks` | Stores each task's metadata (title, timestamps, links, notes, and whether it is marked current). |
| `settings` | Persists the selected sort order. |
| `work_log` | Appends a row each time a task is marked as the current focus. |

This layout keeps the data model simple while supporting future enhancements such as reporting or exporting.

## License

This project is distributed under the terms of the [MIT License](LICENSE).
