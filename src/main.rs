use eframe::App;
use tokio::sync::mpsc;
use crate::file_commander::ui::UI;

mod add_two_numbers;
mod merge_sorted_lists;
mod long_arithmetics;
mod rotate_list;
mod utils;
mod file_commander;


async fn test(){
    // Создаем канал с буфером на 32 сообщения
    let (tx, mut rx) = mpsc::channel(32);

    // Запускаем первую асинхронную задачу
    tokio::spawn(async move {
        for i in 0..10 {
            // Отправляем сообщение в канал
            if let Err(_) = tx.send(i).await {
                println!("Receiver dropped");
                return;
            }
        }
    });

    // Запускаем вторую асинхронную задачу
    tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            // Обрабатываем полученное сообщение
            println!("Received: {}", value);
        }
    });

    // Ждем завершения задач
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await
}

#[tokio::main]
async fn main() -> eframe::Result
{
    test().await;

    //add_two_numbers::solution::new_from_value(3);
    //merge_sorted_lists::solution::merge_sorted_nodes(&None, &None);
    //rotate_list::list_node::ListNode::new(4);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 1024.0]),

        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        multisampling: 4,

        ..Default::default()
    };

    let volumes = file_commander::file_system_service::get_volumes().await;

    return eframe::run_native(
        "egui file_commander app",
        options,
        Box::new(|cc| {
            let ui = Box::new(UI::new(cc, volumes));
            ui.start_monitor();

            return Ok(ui);
        }),
    );
}
