
pub struct GuitarApp {
}

impl Default for GuitarApp {
    fn default() -> Self {
        Self {
        }
    }
}

impl GuitarApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {


        Default::default()
    }
}

impl eframe::App for GuitarApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { } = self;

        egui::Window::new("guitar widget").show(ctx, |ui| {
            ui.add(crate::thing::guitar());
         });

    }
}
