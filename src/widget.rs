use egui::{vec2, pos2};
use egui::{Color32, Stroke, Align2, FontId};


pub fn guitar_ui<'a>(ui: &mut egui::Ui, in_strings: Option<Vec<String>>) -> egui::Response {

    let desired_size = ui.spacing().interact_size.y * egui::vec2(27.0, 9.0); // 7:1?


    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if ui.is_rect_visible(rect) {

        let strings: Vec<String>;

        if in_strings == None {
            strings = ["e", "b", "G", "D", "A", "E"].into_iter().map(ToOwned::to_owned).collect::<Vec<String>>();
        } else {
            strings = in_strings.unwrap();
        }

        let size = vec2( -90.0, -80.0 );
        let mut posn = vec2( -10.0, -70.0 );

        for item in strings.clone() {
            let string = rect
            .expand2(size)
            .translate(posn);

            ui.painter()
            .rect(string, 0.0, Color32::from_rgb(99, 99, 120) , Stroke::default());

            posn = vec2( posn.x , posn.y + 20.0 );

            ui.painter()
            .text(string.translate( vec2(157.0, 0.0) ).center(), Align2::CENTER_CENTER, item, FontId::monospace(9.0), egui::style::Visuals::text_color(ui.visuals()));


        }

        /*
        let frets: usize = 12;
        let size = vec2( -242.0, -30.0 );
        let mut posn = vec2( -162.0 , -20.0 );
        let mut step = 30.0;

        for i in 0..=frets {
            let fret = rect
                        .expand2(size)
                        .translate(posn);
            posn.x += step;
            step -= 1.4;

            if i > 8 {
                step += 1.7;
            }

            ui.painter()
                        .rect(fret, 0.0, Color32::from_rgb(99, 120, 120) , Stroke::default());
            
            ui.painter()
            .text(fret.translate( vec2( 0.0, 70.0 )).center(), Align2::CENTER_CENTER, i, FontId::monospace(9.0), egui::style::Visuals::text_color(ui.visuals()));
        }
         */
        
        let frets: usize = 12;
        let size = vec2( -242.0, -30.0 );
        let mut posn = vec2( 80.0, 10.0 );
        let mut step = 21.0;

        for j in 0..strings.len()-1 {

            step = 21.0;
            for i in 0..=frets {

                let mut fret_segment = rect.clone();
                fret_segment.set_width(2.0);
                fret_segment.set_height(20.0);
                fret_segment = fret_segment.translate( posn );
        
                ui.painter().rect_filled(fret_segment, 0.0, Color32::from_rgb(255,255,255));

                
                posn = vec2( posn.x + step, posn.y + 0.0 ) ;
                step -= 0.7;
            }


            posn = vec2( 80.0, posn.y + 20.0 ) ;
        }
        
    }

    response
}


pub fn guitar(in_strings: Option<Vec<String>>) -> impl egui::Widget {
    let in_strings = in_strings.clone();
    move |ui: &mut egui::Ui| guitar_ui(ui, in_strings)
}
