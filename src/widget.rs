use egui::vec2;
use egui::{Color32, Stroke, Align2, FontId};

#[derive(Copy, Clone, Debug)]
pub struct HeldFret {
    pub finger: i8,
    pub fret: i8,
}

#[derive(Clone, Debug)]
pub struct GuitarString {
    pub string: String,
    pub held: Vec<HeldFret>,
}

#[derive(Clone, Debug)]
pub struct Guitar {
    pub strings: Vec<GuitarString>,
}

pub fn guitar_ui(ui: &mut egui::Ui, in_strings: Option<Guitar>) -> egui::Response {


    let desired_size = ui.spacing().interact_size.y * egui::vec2(27.0, 9.0);


    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());


    if ui.is_rect_visible(rect) {

        
        let frets: usize = 12;
        
        let mut strings: Vec<String> = vec![];
        let mut selected_frets: Vec<Vec<i8>> = vec![];

        match in_strings {
            Some(s) => {
                for guitar_string in s.strings {
                    strings.push(guitar_string.string);
                    let mut selected_frets_in_string: Vec<i8> = vec![];
                    for _i in 0..=frets {
                        selected_frets_in_string.push(-1);
                    }
                    for held_fret in guitar_string.held {
                        selected_frets_in_string[held_fret.fret as usize] = held_fret.finger;
                    }
                    selected_frets.push(selected_frets_in_string);
                }
            },
            None => {
               strings = ["e", "b", "G", "D", "A", "E"].into_iter().map(ToOwned::to_owned).collect::<Vec<String>>();
               for _i in 0..=frets {
                    selected_frets.push(vec![]);
               }
            }
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

                let actual_fret = frets-i;
                
                if selected_frets.len() >= j {
                    if selected_frets[j].len() as isize - 1 >= actual_fret as isize {
                        /*
                        -1 to disable, 0 for no finger. so if you want to have 3rd fret enabled, no finger, -1 ,-1 , 0. if you want to have finger 1 on the 3rd: -1 , -1 , 1
                         */
                        let finger = selected_frets[j][actual_fret];
                        if finger != -1 {
                            ui.painter().circle_filled(fret_segment.center_top(), 5.0, Color32::from_rgb(255,255,255));
                            if finger != 0 {
                                ui.painter().text(fret_segment.center_top(), Align2::CENTER_CENTER, selected_frets[j][actual_fret], FontId::monospace(9.0), Color32::from_rgb(0,0,0));
                            }
                        }
                        
                    }
                }


                posn = vec2( posn.x + distance, posn.y + 0.0 ) ;

                if j == strings.len()-2 { // if we're on the last string, print the fret numbers
                    ui.painter()
                    .text(fret_segment.translate( vec2( 0.0, 20.0 )).center(), Align2::CENTER_CENTER, actual_fret, FontId::monospace(9.0), egui::style::Visuals::text_color(ui.visuals()));
                }
            }

            posn = vec2( 80.0, posn.y + 20.0 ) ;
        }

    }

    response
}


pub fn guitar(in_strings: Option<Guitar>) -> impl egui::Widget {
    let in_strings = in_strings;
    move |ui: &mut egui::Ui| guitar_ui(ui, in_strings)
}
