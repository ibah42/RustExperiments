use crate::file_commander::table_demo::TableDemo;

mod add_two_numbers;
mod merge_sorted_lists;
mod long_arithmetics;
mod rotate_list;
mod utils;
mod file_commander;

#[tokio::main]
async fn main() -> eframe::Result
{
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

    file_commander::file_system_service::get_root();

    run().await;

    return eframe::run_native(
        "egui file_commander app",
        options,
        Box::new(|cc| Ok(Box::new(TableDemo::new(cc)))),
    );
}

async fn run()
{
    let volumes = file_commander::file_system_service::get_volumes().await;

    for volume in volumes {
        println!("volumes: {:#?}", volume);
    }
}

