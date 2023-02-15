

use crate::widget::*;

#[derive(Default)]
pub struct GuitarApp {
    string: String,
}


impl GuitarApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for GuitarApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {


        let mut selected = Guitar {
            strings: vec![
                GuitarString {
                    string: "e".to_string(),
                    held: vec![
                        HeldFret {
                            finger: 2,
                            fret: 3,
                        }
                    ],
                },
                GuitarString {
                    string: "b".to_string(),
                    held: vec![],
                },
                GuitarString {
                    string: "G".to_string(),
                    held: vec![],
                },
                GuitarString {
                    string: "D".to_string(),
                    held: vec![
                        HeldFret {
                            finger: 2,
                            fret: 5,
                        }
                    ],
                },
                GuitarString {
                    string: "A".to_string(),
                    held: vec![],
                },
                GuitarString {
                    string: "E".to_string(),
                    held: vec![],
                }
            ],
        };

        egui::Window::new("guitar widget").show(ctx, |ui| {
            ui.add(crate::widget::guitar(Some(selected)));


         });

         egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                
                ui.text_edit_singleline(&mut self.string);

                let mut guitar = Guitar {
                    strings: vec![],
                };

                // self.string.split(',').map(ToOwned::to_owned).collect::<Vec<String>>()

                ui.add(crate::widget::guitar( None ));
            });

        });

    }
}
