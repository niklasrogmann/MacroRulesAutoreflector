extern crate MacroRulesAutoreflector;
use MacroRulesAutoreflector::Autoreflect;

#[derive(Autoreflect)]
// (macro_rules! macro), function name)
#[set_for_field_derive(for_each_field, fn_name)]
struct AppData {
    label_data: String,
    checkbox_data: bool,
    clicked_count: u64,
    iu8: u8,
    spr32: f32,
    stepper: f64,
    editable_text: String,
}

macro_rules! for_each_field {
    (bool; name : ident; b : block;) => {
        
    };
    (_) => {}
}

fn main() {
    println!("Hello, world!");
    fn_name();
}

