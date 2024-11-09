use eframe::App;
use crate::file_commander::ui::UI;

mod utils;
mod file_commander;



#[tokio::main]
async fn main() -> eframe::Result
{
    utils::async_examples::test_channel().await;
    utils::async_examples::test_events().await;
    utils::async_examples::test_mutex().await;
    utils::async_examples::test_locks().await;

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
