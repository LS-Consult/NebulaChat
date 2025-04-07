#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]
use crate::ui::components::topbar_header::{create as create_topbar_header };
use crate::ui::components::leftbar_server_list::{create as create_leftbar_server_list, AppState};

mod ui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        centered: true,
        ..Default::default()
    };
    
    let mut state = AppState::default();
    
    eframe::run_simple_native("Nebula Chat", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            create_topbar_header(ui);
            create_leftbar_server_list(ui, &mut state);
        });
    })
}
