use egui::{vec2, pos2};
use egui::{Color32, Stroke, Align2, FontId};


pub fn guitar_ui(ui: &mut egui::Ui) -> egui::Response {

    let desired_size = ui.spacing().interact_size.y * egui::vec2(27.0, 9.0); // 7:1?


    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if ui.is_rect_visible(rect) {

        let strings = vec![ "e", "b", "G", "D", "A", "E" ];

        let size = vec2( -90.0, -80.0 );
        let mut posn = vec2( -10.0, -70.0 );

        for i in 0..strings.len() {
            let string = rect
            .expand2(size)
            .translate(posn);

            ui.painter()
            .rect(string, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());

            posn = vec2( posn.x , posn.y + 20.0 );

            ui.painter()
            .text(string.translate( vec2(157.0, 0.0) ).center(), Align2::CENTER_CENTER, strings[i], FontId::monospace(9.0), egui::style::Visuals::text_color(&ui.visuals()));


        }

        let frets: usize = 12;
        let size = vec2( -242.0, -30.0 );
        let mut posn = vec2( -140.0 , -20.0 );
        let mut step = 30.0;

        for i in 0..=frets {
            let fret = rect
                        .expand2(size)
                        .translate(posn);
            posn.x += step;
            step -= 1.4;
            ui.painter()
                        .rect(fret, 0.0, Color32::from_rgb(99, 120, 120) , Stroke::default());
            
            ui.painter()
            .text(fret.translate( vec2( 0.0, 70.0 )).center(), Align2::CENTER_CENTER, i, FontId::monospace(9.0), egui::style::Visuals::text_color(&ui.visuals()));
        }

    }

    response
}


pub fn guitar() -> impl egui::Widget {
    move |ui: &mut egui::Ui| guitar_ui(ui)
}
