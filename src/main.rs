slint::include_modules!();

fn main() {
    let tasks = Vec::new();

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

    ui.run();
}
