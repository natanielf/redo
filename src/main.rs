use slint::Model;
use std::rc::Rc;

slint::include_modules!();

fn main() {
    let tasks = Rc::new(slint::VecModel::<Task>::from(Vec::new()));

    let ui = AppWindow::new();

    ui.on_add_task({
        let mut tasks = tasks.clone();
        move |text| {
            tasks.push(Task {
                text,
                is_completed: false,
            })
        }
    });

    ui.on_remove_completed({
        let tasks = tasks.clone();
        move || {
            let mut offset = 0;
            for i in 0..tasks.row_count() {
                if tasks.row_data(i - offset).unwrap().is_completed {
                    tasks.remove(i - offset);
                    offset += 1;
                }
            }
        }
    });

    ui.set_tasks(tasks.into());

    ui.run();
}
