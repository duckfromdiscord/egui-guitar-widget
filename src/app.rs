

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

        egui::Window::new("guitar widget").show(ctx, |ui| {
            ui.add(crate::widget::guitar(None));


         });

         egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                
                ui.text_edit_singleline(&mut self.string);
                ui.add(crate::widget::guitar(Some( self.string.split(',').into_iter().map(ToOwned::to_owned).collect::<Vec<String>>() )));
            });

        });

    }
}
