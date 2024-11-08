use std::default::Default;
use eframe::CreationContext;
use egui::{Color32, Separator, TextStyle, TextWrapMode, Widget};
use egui::WidgetText::RichText;
use egui::WidgetType::Label;
use egui_extras;
use crate::file_commander::volume::Volume;
use crate::file_commander::disk_type::DiskType;
use crate::file_commander::file_system_service::*;
use crate::utils::time_utils;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UI {
    striped: bool,
    resizable: bool,
    clickable: bool,
    num_rows: usize,
    scroll_to_row_slider: usize,
    scroll_to_row: Option<usize>,
    selection: std::collections::HashSet<usize>,
    checked: bool,
    reversed: bool,
    volumes: Vec<Volume>,
}


impl UI {
    async fn monitor() {
        let timer = std::time::Instant::now();
        loop {
            tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;

            time_utils::print_time_spent(&timer, "run monitor");
        }
    }

    pub fn start_monitor(&self){
        tokio::task::spawn(UI::monitor());
    }

}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx,
            |ui|
            {
                ui.heading(self.name());
                Self::show_ui(self, ui)
            }
        );

        ctx.clone().request_repaint();

        /*egui::Window::new(self.name())
            .open(&mut true)
            .default_width(800.0)
            .show(ctx, |ui| Self::show_ui(self, ui) );

         */
    }

/*
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {

            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!(
                "../../../crates/egui/assets/ferris.png"
            ));
        });
    }
    */

}
impl UI
{
    pub fn new(cc: & CreationContext, volumes: Vec<Volume>) -> Self {

        /*
        cc.egui_ctx.style_mut( |&mut style| {
            style.override_text_color = Some(egui::Color32::RED);
            style.override_text_style = Some(egui::TextStyle::Monospace);
            style.wrap_mode = Some(TextWrapMode::Truncate);
        });
        */

        Self {
            striped: true,
            resizable: true,
            clickable: true,
            num_rows: 10_000,
            scroll_to_row_slider: 0,
            scroll_to_row: None,
            selection: Default::default(),
            checked: false,
            reversed: false,
            volumes: volumes,
        }
    }

    pub fn name(&self) -> &'static str {
        "☰ Table"
    }

    fn show_ui(&mut self, ui: &mut egui::Ui)
    {
        let mut reset = false;

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.striped, "Striped");
                ui.checkbox(&mut self.resizable, "Resizable columns");
                ui.checkbox(&mut self.clickable, "Clickable rows");
            });

            ui.horizontal(|ui| {
                for volume in & self.volumes {
                    Separator::default().grow(4.0).ui(ui);
                    egui::widgets::Button::new(
                        egui::RichText::new(volume.get_drive_name()).monospace().color(egui::Color32::DARK_GREEN).size(15.0),
                    ).ui(ui);
                }
            });

            ui.label("Table type:");
            ui.add(
                egui::Slider::new(&mut self.num_rows, 0..=100_000)
                    .logarithmic(true)
                    .text("Num rows"),
            );

            let slider_response = ui.add(
                egui::Slider::new(&mut self.scroll_to_row_slider, 0..=self.num_rows)
                    .logarithmic(true)
                    .text("Row to scroll to"),
            );
            if slider_response.changed() {
                self.scroll_to_row = Some(self.scroll_to_row_slider);
            }

            reset = ui.button("Reset").clicked();
        });

        ui.separator();

        // Leave room for the source code link after the table file_commander:
        let body_text_size = TextStyle::Body.resolve(ui.style()).size;
        use egui_extras::{Size, StripBuilder};
        egui_extras::StripBuilder::new(ui)
            .size(egui_extras::Size::remainder().at_least(100.0)) // for the table
            .size(egui_extras::Size::exact(body_text_size)) // for the source code link
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui, reset);
                    });
                });
                strip.cell(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(crate::egui_github_link_file!());
                    });
                });
            });
    }

    fn table_ui(&mut self, ui: &mut egui::Ui, reset: bool) {
        use egui_extras::{Column, TableBuilder};

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();
        let mut table = TableBuilder::new(ui)
            .striped(self.striped)
            .resizable(self.resizable)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::remainder())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height);


        if self.clickable {
            table = table.sense(egui::Sense::click());
        }

        if let Some(row_index) = self.scroll_to_row.take() {
            table = table.scroll_to_row(row_index, None);
        }

        if reset {
            table.reset();
        }

        table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Row");
                        },
                        |ui| {
                            self.reversed ^= ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });
                header.col(|ui| {
                    egui::widgets::Label::new(
                        egui::RichText::new("name")
                            .color(Color32::from_rgb(2, 20, 1))
                            .background_color(Color32::from_rgba_premultiplied(0,20,0,10))
                            .italics()
                            .strikethrough()
                    ).ui(ui);
                });
                header.col(|ui| {
                    ui.strong("Expanding content");
                });
                header.col(|ui| {
                    ui.strong("Interaction");
                });
                header.col(|ui| {
                    ui.strong("Content");
                });
            })
            .body(|mut body|
                {
                    let row_height = |i: usize| if thick_row(i) { 30.0 } else { 18.0 };

                    body.heterogeneous_rows((0..self.num_rows).map(row_height), |mut row| {
                        let row_index = if self.reversed {
                            self.num_rows - 1 - row.index()
                        } else {
                            row.index()
                        };

                        row.set_selected(self.selection.contains(&row_index));

                        row.col(|ui| {
                            ui.label(row_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label(long_text(row_index));
                        });
                        row.col(|ui| {
                            expanding_content(ui);
                        });
                        row.col(|ui| {
                            ui.checkbox(&mut self.checked, "Click me");
                        });
                        row.col(|ui| {
                            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                            if thick_row(row_index) {
                                ui.heading("Extra thick row");
                            } else {
                                ui.label("Normal row");
                            }
                        });

                        self.toggle_row_selection(row_index, &row.response());
                    });

            });
    }

    fn toggle_row_selection(&mut self, row_index: usize, row_response: &egui::Response) {
        if row_response.clicked() {
            if self.selection.contains(&row_index) {
                self.selection.remove(&row_index);
            } else {
                self.selection.insert(row_index);
            }
        }
    }
}

fn expanding_content(ui: &mut egui::Ui) {
    ui.add(egui::Separator::default().horizontal());
}

fn long_text(row_index: usize) -> String {
    format!("Row {row_index} has some long text that you may want to clip, or it will take up too much horizontal space!")
}

fn thick_row(row_index: usize) -> bool {
    row_index % 6 == 0
}


/// Create a [`Hyperlink`](egui::Hyperlink) to this egui source code file on github.
#[macro_export]
macro_rules! egui_github_link_file {
    () => {
        $crate::egui_github_link_file!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}

/// Create a [`Hyperlink`](egui::Hyperlink) to this egui source code file and line on github.
#[macro_export]
macro_rules! egui_github_link_file_line {
    () => {
        $crate::egui_github_link_file_line!("(source code)")
    };
    ($label: expr) => {
        egui::github_link_file_line!(
            "https://github.com/emilk/egui/blob/master/",
            egui::RichText::new($label).small()
        )
    };
}
