use egui::pos2;
use egui::Rect;
use egui::vec2;
use egui::Color32;
use egui::Stroke;

pub fn guitar_ui(ui: &mut egui::Ui) -> egui::Response {

    let desired_size = ui.spacing().interact_size.y * egui::vec2(27.0, 9.0); // 7:1?


    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if ui.is_rect_visible(rect) {


        let hi_e = rect
                        .expand2(vec2( -90.0, -80.0 ))
                        .translate(vec2(-10.0, -70.0));
        let hi_b = rect
                        .expand2(vec2( -90.0, -80.0 ))
                        .translate(vec2(-10.0, -50.0));  
        let low_g = rect
                        .expand2(vec2( -90.0, -80.0 ))
                        .translate(vec2(-10.0, -30.0));
        let low_d = rect
                        .expand2(vec2( -90.0, -80.0 ))
                        .translate(vec2(-10.0, -10.0)); 
        let low_a = rect
                        .expand2(vec2( -90.0, -80.0 ))
                        .translate(vec2(-10.0, 10.0));
        let low_e = rect
                        .expand2(vec2( -90.0, -80.0 ))
                        .translate(vec2(-10.0, 30.0));  
        
        ui.painter()
            .rect(hi_e, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());
        ui.painter()
            .rect(hi_b, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());
        ui.painter()
            .rect(low_g, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());
        ui.painter()
            .rect(low_d, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());
        ui.painter()
            .rect(low_a, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());
        ui.painter()
            .rect(low_e, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());


        for i in 0..14 {
            let fret = rect
                        .expand2(vec2( -242.0, -30.0 ))
                        .translate(vec2(130.0-(i as f32* (8.0+(i as f32)) ) , -20.0+(0.0) ));
            ui.painter()
                        .rect(fret, 0.0, Color32::from_rgb(99, 120, 120) , Stroke::default());
        }

    }

    response
}


pub fn guitar() -> impl egui::Widget {
    move |ui: &mut egui::Ui| guitar_ui(ui)
}
