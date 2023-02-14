use egui::vec2;
use egui::{Color32, Stroke, Align2, FontId};


pub fn guitar_ui(ui: &mut egui::Ui, in_strings: Option<Vec<String>>) -> egui::Response {


    let desired_size = ui.spacing().interact_size.y * egui::vec2(27.0, 9.0);


    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if ui.is_rect_visible(rect) {

        let strings: Vec<String> = if Option::is_none(&in_strings) {
            ["e", "b", "G", "D", "A", "E"].into_iter().map(ToOwned::to_owned).collect::<Vec<String>>()
        } else {
            in_strings.unwrap()
        };


        match &*strings {
            [] => return response,
            [x] => {
                if x.is_empty() {
                    return response;
                }
            },
            _ => (),
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
        
        let frets: usize = 12;
        let mut posn = vec2( 80.0, 10.0 );


        // scaling code adapted from http://www.buildyourguitar.com/resources/tips/fretdist.htm
        let mut distance;
        let scale = 80.0;
        let mut location;
        let mut scaling_factor;
        

        for j in 0..strings.len()-1 {
            distance = 0.0;
            for i in 0..=frets {

                location = scale - distance;
                scaling_factor = location / 17.817;
                distance += scaling_factor;

                let mut fret_segment = rect;
                fret_segment.set_width(2.0);
                fret_segment.set_height(20.0);
                fret_segment = fret_segment.translate( posn );
        
                ui.painter().rect_filled(fret_segment, 0.0, Color32::from_rgb(255,255,255));
                
                posn = vec2( posn.x + distance, posn.y + 0.0 ) ;

                if j == strings.len()-2 { // if we're on the last string, print the fret numbers
                    ui.painter()
                    .text(fret_segment.translate( vec2( 0.0, 20.0 )).center(), Align2::CENTER_CENTER, frets-i, FontId::monospace(9.0), egui::style::Visuals::text_color(ui.visuals()));
                }
            }

            posn = vec2( 80.0, posn.y + 20.0 ) ;
        }

    }

    response
}


pub fn guitar(in_strings: Option<Vec<String>>) -> impl egui::Widget {
    let in_strings = in_strings;
    move |ui: &mut egui::Ui| guitar_ui(ui, in_strings)
}
