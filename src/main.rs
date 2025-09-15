use slint::{ComponentHandle, ModelRc, VecModel, Model};
use rusqlite::{Connection, params};
use chrono::Local;
use std::{rc::Rc, cell::RefCell};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

slint::include_modules!();

#[derive(Clone)]
struct TaskData {
    id: i64,
    title: String,
    created: String,
    tracker: String,
    branch: String,
    review: String,
    commit: String,
    notes: String,
    current: bool,
}

fn init_db(conn: &Connection) {
    conn.execute("CREATE TABLE IF NOT EXISTS tasks (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, created TEXT, tracker_url TEXT, branch TEXT, review_url TEXT, commit_template TEXT, notes TEXT, current INTEGER)", []).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL)", []).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS work_log (id INTEGER PRIMARY KEY AUTOINCREMENT, task_id INTEGER, start_time TEXT)", []).unwrap();
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM tasks", [], |r| r.get(0)).unwrap();
    if count == 0 {
        let now = Local::now().to_rfc3339();
        conn.execute("INSERT INTO tasks (title, created, tracker_url, branch, review_url, commit_template, notes, current) VALUES (?,?,?,?,?,?,?,0)",
            params!["Sample Task", now, "", "", "", "", "",]).unwrap();
    }
}

fn get_sort_order(conn: &Connection) -> i32 {
    conn.query_row("SELECT value FROM settings WHERE key='sort_order'", [], |r| r.get::<_, String>(0)).ok()
        .and_then(|v| v.parse().ok()).unwrap_or(0)
}

fn save_sort_order(conn: &Connection, order: i32) {
    conn.execute("INSERT INTO settings(key,value) VALUES('sort_order', ?) ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![order]).unwrap();
}

fn load_tasks(conn: &Connection, order: i32) -> Vec<TaskData> {
    let sql = if order == 0 {
        "SELECT id,title,created,tracker_url,branch,review_url,commit_template,notes,current FROM tasks ORDER BY title"
    } else {
        "SELECT id,title,created,tracker_url,branch,review_url,commit_template,notes,current FROM tasks ORDER BY created"
    };
    let mut stmt = conn.prepare(sql).unwrap();
    let rows = stmt.query_map([], |row| Ok(TaskData {
        id: row.get(0)?,
        title: row.get(1)?,
        created: row.get(2)?,
        tracker: row.get(3)?,
        branch: row.get(4)?,
        review: row.get(5)?,
        commit: row.get(6)?,
        notes: row.get(7)?,
        current: row.get::<_, i64>(8)? != 0,
    })).unwrap();
    let mut vec = Vec::new();
    for r in rows { vec.push(r.unwrap()); }
    vec
}

fn load_task_detail(conn: &Connection, id: i64) -> TaskData {
    conn.query_row("SELECT id,title,created,tracker_url,branch,review_url,commit_template,notes,current FROM tasks WHERE id=?", params![id], |row| {
        Ok(TaskData {
            id: row.get(0)?,
            title: row.get(1)?,
            created: row.get(2)?,
            tracker: row.get(3)?,
            branch: row.get(4)?,
            review: row.get(5)?,
            commit: row.get(6)?,
            notes: row.get(7)?,
            current: row.get::<_, i64>(8)? != 0,
        })
    }).unwrap()
}

fn main() {
    let db_path = "tasks.db".to_string();
    let conn = Rc::new(Connection::open(&db_path).unwrap());
    init_db(&conn);
    let sort_order = get_sort_order(&conn);
    let tasks_vec = load_tasks(&conn, sort_order);
    let model = Rc::new(VecModel::from(
        tasks_vec.iter().map(|t| Task { id: t.id as i32, title: t.title.clone().into(), current: t.current }).collect::<Vec<_>>()
    ));
    let app = MainWindow::new().unwrap();
    app.set_tasks(ModelRc::new(model.clone()));
    app.set_sort_order(sort_order);

    let task_data = Rc::new(RefCell::new(tasks_vec));

    {
        let conn = conn.clone();
        let model = model.clone();
        let task_data = task_data.clone();
        app.on_sort_changed(move |order| {
            save_sort_order(&conn, order);
            let tasks = load_tasks(&conn, order);
            model.set_vec(tasks.iter().map(|t| Task { id: t.id as i32, title: t.title.clone().into(), current: t.current }).collect::<Vec<_>>());
            *task_data.borrow_mut() = tasks;
        });
    }

    {
        let app_weak = app.as_weak();
        let db_path = db_path.clone();
        let task_data = task_data.clone();
        app.on_select_task(move |idx| {
            if idx < 0 { return; }
            let task = task_data.borrow()[idx as usize].clone();
            let app_weak2 = app_weak.clone();
            let show_loader = Arc::new(AtomicBool::new(true));
            slint::Timer::single_shot(std::time::Duration::from_millis(100), {
                let app_weak = app_weak2.clone();
                let show_loader = show_loader.clone();
                move || {
                    if show_loader.load(Ordering::Relaxed) {
                        if let Some(app) = app_weak.upgrade() {
                            app.set_loading(true);
                        }
                    }
                }
            });
            let db_path2 = db_path.clone();
            let show_loader2 = show_loader.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(200));
                let thread_conn = Connection::open(&db_path2).unwrap();
                let detail = load_task_detail(&thread_conn, task.id);
                show_loader2.store(false, Ordering::Relaxed);
                slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak2.upgrade() {
                        app.set_loading(false);
                        app.set_detail_title(detail.title.into());
                        app.set_detail_created(detail.created.into());
                        app.set_detail_tracker(detail.tracker.into());
                        app.set_detail_branch(detail.branch.into());
                        app.set_detail_review(detail.review.into());
                        app.set_detail_commit(detail.commit.into());
                        app.set_detail_notes(detail.notes.into());
                    }
                }).unwrap();
            });
        });
    }

    {
        let app_weak = app.as_weak();
        let conn = conn.clone();
        let task_data = task_data.clone();
        let model = model.clone();
        app.on_save_detail(move || {
            if let Some(app) = app_weak.upgrade() {
                let idx = app.get_selected_index();
                if idx >= 0 {
                    let id = task_data.borrow()[idx as usize].id;
                    let title = app.get_detail_title().to_string();
                    let tracker = app.get_detail_tracker().to_string();
                    let branch = app.get_detail_branch().to_string();
                    let review = app.get_detail_review().to_string();
                    let commit = app.get_detail_commit().to_string();
                    let notes = app.get_detail_notes().to_string();
                    conn.execute("UPDATE tasks SET title=?, tracker_url=?, branch=?, review_url=?, commit_template=?, notes=? WHERE id=?",
                        params![title, tracker, branch, review, commit, notes, id]).unwrap();
                    {
                        let mut data = task_data.borrow_mut();
                        let t = &mut data[idx as usize];
                        t.title = title.clone();
                        t.tracker = tracker;
                        t.branch = branch;
                        t.review = review;
                        t.commit = commit;
                        t.notes = notes;
                        model.set_row_data(idx as usize, Task { id: id as i32, title: t.title.clone().into(), current: t.current });
                    }
                }
            }
        });
    }

    {
        let app_weak = app.as_weak();
        let conn = conn.clone();
        let task_data = task_data.clone();
        let model = model.clone();
        app.on_set_current(move || {
            if let Some(app) = app_weak.upgrade() {
                let idx = app.get_selected_index();
                if idx >= 0 {
                    let id = task_data.borrow()[idx as usize].id;
                    conn.execute("UPDATE tasks SET current=0", []).unwrap();
                    conn.execute("UPDATE tasks SET current=1 WHERE id=?", params![id]).unwrap();
                    let now = Local::now().to_rfc3339();
                    conn.execute("INSERT INTO work_log(task_id,start_time) VALUES(?,?)", params![id, now]).unwrap();
                    for (i, t) in task_data.borrow_mut().iter_mut().enumerate() {
                        t.current = t.id == id;
                        model.set_row_data(i, Task { id: t.id as i32, title: t.title.clone().into(), current: t.current });
                    }
                }
            }
        });
    }

    {
        app.on_about(|| {
            let about = AboutWindow::new().unwrap();
            let _ = about.run();
        });
    }

    app.set_selected_index(-1);
    app.run().unwrap();
}

