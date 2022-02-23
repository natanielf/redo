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

    ui.set_tasks(tasks.into());

    ui.run();
}
