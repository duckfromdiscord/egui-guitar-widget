

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

                let string_names = self.string.split(',').collect::<Vec<&str>>();

                if !string_names[0].is_empty() || string_names.len() > 1 { // you can have the first string name empty and still have other strings to make
                    for string_name in string_names {
                        guitar.strings.push( GuitarString {
                            held: vec![],
                            string: string_name.to_string()
                        });
                    }
                    ui.add(crate::widget::guitar( Some(guitar) ));
                } else {
                    ui.add(crate::widget::guitar( None ));
                }
                
            });

        });

    }
}
