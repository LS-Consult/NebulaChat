use eframe::{egui};
use egui::{Align, Layout, Ui};

#[derive(PartialEq)]
enum SelectedApp {
    None,
    HeldStudios,
    LSConsult,
}

pub struct AppState {
    selected: SelectedApp,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            selected: SelectedApp::None,
        }
    }
}

pub fn create(ui: &mut Ui, state: &mut AppState) {
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.vertical(|ui| {
            ui.add_space(8.0);

            if ui.button("Held Studios").clicked() {
                state.selected = SelectedApp::HeldStudios;
            }

            ui.add_space(8.0);

            if ui.button("LS Consult").clicked() {
                state.selected = SelectedApp::LSConsult;
            }

            ui.add_space(8.0);
        });

        ui.separator();

        ui.vertical(|ui| {
            match state.selected {
                SelectedApp::None => {
                    ui.label("Selecione uma opção ao lado.");
                }
                SelectedApp::HeldStudios => {
                    ui.label("Você clicou em Held Studios!");
                }
                SelectedApp::LSConsult => {
                    ui.label("Você clicou em LS Consult!");
                }
            }
        });
    });
}
