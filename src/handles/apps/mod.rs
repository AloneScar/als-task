use crate::models::{App, InputMode, ScrollY, Task, Window};

use crate::database::{init_database, get_all_data};

mod movement;
mod modify;


impl App {
    pub fn new() -> App {
        let conn = init_database().unwrap();
        let mut task_groups = get_all_data(&conn).unwrap();
        task_groups[0].tasks = vec![
            Task {content: "fsadfsad".to_string(), ..Default::default()},
            Task {content: "fsadfsad".to_string(), ..Default::default()},
            Task {content: "fsadfsad".to_string(), ..Default::default()},
            Task {content: "fsadfsad".to_string(), ..Default::default()},
            Task {content: "fsadfsad".to_string(), ..Default::default()},
        ];
        App {
            conn,
            task_groups,
            input_mode: InputMode::Normal,
            index: 0,
            scroll_task: ScrollY { current: 0, max: 0 },
            scroll_group: ScrollY { current: 0, max: 0 },
            window: Window::Groups,
        }
    }
}