# Roadmap

This roadmap captures a forward-looking plan for the Rust Slint Task Manager. It pairs an honest assessment of the current state with a staged set of improvements designed to make the application a reliable companion for day-to-day development work.

## Current Assessment

### Strengths
- **Solid foundations:** A clear Slint UI and a lightweight SQLite schema already support the core workflow of creating, editing, and focusing on tasks.
- **Responsive interactions:** The detail pane uses background loading and progress feedback to keep the interface responsive even when the database query is slow.
- **Work-log data model:** Each "set current" action is timestamped, providing data that can later power focus analytics.

### Key Gaps
- **Limited task metadata:** There are no due dates, priorities, tags, or status values to help users triage work.
- **Minimal workflow support:** Users cannot search, filter, bulk-edit, or archive tasks; the work log is collected but never surfaced to the user.
- **Error handling and resilience:** Database access assumes success, the `tasks.db` file lives beside the binary with no backup, and the UI does not surface failures.
- **Testing and modularity:** Application logic sits in `main.rs` without unit tests or modules, making it hard to extend safely.

## Strategic Themes
1. **Polish the fundamentals** – improve reliability, validation, and the editing experience.
2. **Unlock productivity features** – add planning aids such as prioritisation, reminders, and quick navigation.
3. **Leverage collected data** – transform the existing work-log entries into insights and reporting.
4. **Prepare for distribution** – package the app so contributors and teammates can try it without a Rust toolchain.

## Near-Term Goals (v0.2 – Foundations)
- Introduce structured error handling (`anyhow`, `thiserror`) and surface failures to the UI.
- Split database code into a `storage` module with repository-style functions plus integration tests using temporary SQLite files.
- Replace the unconditional sample task seeding with a guided empty state inside the UI.
- Add validation to prevent duplicate titles and to normalise URLs before saving.
- Ensure `cargo fmt`, `cargo clippy`, and basic unit tests run in CI (GitHub Actions or similar).

## Mid-Term Goals (v0.3 – Productivity Boost)
- Extend the schema with optional fields: priority (enum), due date (`datetime`), status (`active`, `blocked`, `done`), and free-form tags stored in a join table.
- Implement search, filters, and sort toggles in the UI to help users find tasks quickly.
- Allow bulk operations (archive, delete) and add soft-deletion support so the database never loses history instantly.
- Add a work-log viewer that summarises daily/weekly focus time per task and allows exporting CSV.
- Introduce keyboard shortcuts (e.g., `Cmd/Ctrl+N` to add a task, arrow keys to navigate the list).

## Long-Term Vision (v0.4+)
- Package the app for Windows, macOS, and Linux using cross-platform bundlers (e.g., `cargo bundle`, `tauri-bundler`, or platform-specific tooling).
- Support optional cloud sync by abstracting the storage layer and adding a remote backend (e.g., REST or gRPC service).
- Enable integrations with popular issue trackers (Jira, GitHub) by storing authentication tokens securely and syncing metadata.
- Provide configurable notifications and reminders, leveraging the operating system notification APIs.
- Offer extension hooks (command palette, plugin interface) so advanced users can script custom workflows.

## Technical Improvement Backlog
- **Refactor UI bindings:** Replace manual property synchronization with view-model structs to reduce boilerplate and keep logic testable.
- **Performance profiling:** Benchmark load times for large task lists and add pagination or incremental loading if required.
- **Accessibility:** Audit contrast ratios, provide larger text presets, and add keyboard focus indicators to meet accessibility guidelines.
- **Localization:** Externalise user-facing strings and add language selection support.
- **Documentation:** Expand README with architecture diagrams and contribute setup instructions for contributors on each platform.

## Measuring Success
- Track adoption metrics such as the number of tasks created, retention of the `current` marker, and the frequency of work-log usage (via opt-in telemetry or manual reporting).
- Define performance targets (e.g., initial load under 200 ms for 500 tasks) and automate regression detection in CI.
- Gather qualitative feedback through GitHub Discussions or surveys after each milestone release.

## Risks & Mitigations
- **Scope creep:** Prioritise roadmap items quarterly and cut features that do not advance the strategic themes.
- **Data loss:** Before implementing advanced features, ship an export/import flow (JSON or CSV) so users can back up their data.
- **Contributor onboarding:** Maintain up-to-date documentation and add a `CONTRIBUTING.md` guide to reduce friction for new collaborators.

The roadmap is a living document—review it after each release to celebrate progress, adjust priorities, and ensure the project continues to serve its users effectively.
