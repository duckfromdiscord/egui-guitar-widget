

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


        let mut selected = vec![ /* e */ vec![1,-1,-1,3], /* b */ vec![0], /* G */ vec![], /* D */ vec![] , /* A */ vec![], /* E */ vec![] ];

        egui::Window::new("guitar widget").show(ctx, |ui| {
            ui.add(crate::widget::guitar(None, Some(selected)));


         });

         egui::CentralPanel::default().show(ctx, |ui| {
            
            ui.horizontal(|ui| {
                
                ui.text_edit_singleline(&mut self.string);

                ui.add(crate::widget::guitar(Some( self.string.split(',').map(ToOwned::to_owned).collect::<Vec<String>>() ), None ));
            });

        });

    }
}
