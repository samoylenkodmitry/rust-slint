# Rust Slint Task Manager

A desktop task manager built with [Slint](https://slint.dev/) and Rust. The application stores tasks locally in an SQLite database and provides a compact dashboard to review task metadata, edit details, and mark the item you are currently working on. A lightweight work-log table captures when a task is marked as current, laying the groundwork for future reporting.

[View the demo video](demo.webm)

## Features

- **Task overview:** Browse all saved tasks, sort by title or creation date, and keep a single "current" task highlighted in the list.
- **Editable detail pane:** Update tracker links, branches, review URLs, commit message templates, and free-form notes.
- **Work focus tracking:** Setting a task as current records a timestamp in the `work_log` table for later analytics.
- **Responsive UI:** The interface is authored in Slint, giving a modern look-and-feel with loading states while the detail pane fetches data.

## Project Structure

```
.
├── src/main.rs        # Application logic, database interactions, Slint callbacks
├── ui/main.slint      # Declarative UI layout and styling
├── assets/            # Images used in the UI
├── build.rs           # Slint build integration
└── tasks.db           # SQLite database (created at runtime)
```

## Getting Started

1. **Install dependencies**
   - Rust toolchain (stable) from [rustup.rs](https://rustup.rs/).
   - Native dependencies for SQLite (pre-installed on most systems).
2. **Run the application**

   ```bash
   cargo run
   ```

   A `tasks.db` file is created automatically in the project root the first time the application runs.

3. **Resetting the database**
   - Delete the generated `tasks.db` file to start from a clean slate.

## Roadmap

Curious about where the project is headed next? Check out [ROADMAP.md](ROADMAP.md) for an in-depth look at planned improvements, from foundational polish to long-term strategic ideas.

## Contributing

Contributions are welcome! Feel free to open an issue to discuss ideas or submit a pull request with enhancements. Please accompany changes with context and testing notes to keep the project healthy.

## License

Licensed under the [MIT License](LICENSE).
