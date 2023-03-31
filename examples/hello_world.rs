
extern crate macro_rules_autoreflector;
use macro_rules_autoreflector::Autoreflect;

#[derive(Autoreflect)]
// (macro_rules! macro), function name)
// TODO: not function, should be macrorules as well to allow nesting and passing around params
#[set_for_field_derive(print_each_field, asdfasdfadsf)]
#[set_for_field_derive(for_each_field, fn_name)]
struct MyData {
    some_text: String,
    a_bool: bool,
    an_u64: u64,
    an_u8: u8,
    an_i8 : i8,
    an_i16 : i16,
    a_ff32: f32,
    a_ff64: f64,
    more_text: String,
    optional_thing: Option<String>
}

/// the autoreflector will call this macro for each field
/// with a bunch of parameters to allow catching specific information
/// about fields
macro_rules! print_each_field {
    // the "$($_ : tt,)*" skips any further field information and is mandatory
    // to keep up with additional reflection data becoming available with future versions
    ([$field : ident, $field_ty : ty, $meta1 : tt, $meta2 : tt, $($_ : tt,)*]; $my_ident_param : ident) => {
        println!("xxx"); // TODO: string stuff
    };
}

/// this example shows how to catch on to / capture information
/// it also allows adding custom parameters such as "$my_ident_param" in this example
/// everything autoreflection related is contained in "[]"
macro_rules! for_each_field {
    // catch bools and capture their names
    
    ([$field : ident, bool, $($_ : tt,)*]; $my_ident_param : ident) => {
        
    };
    // catch whatever field is called some_text
    ([some_text, $field_ty : ty, $($_ : tt,)*]; $my_ident_param : ident) => (
        println!("there is a field called some_text of type "); // TODO: stuff to string conversion
    );
    // catch u64s and print out their value
    // note that the type meta information "unsigned" and "64 bits"
    // do not need to be explicitly matched due to catch-discarding anything else with $($_ : tt,)*
    ([$field : ident, u64, /* u, 64,  */ $($_ : tt,)*]; $my_ident_param : ident) => {
        println!("{}", $my_ident_param.$field);
    };
    // catch any other u* by capturing the field type and specifying the type u,
    // then further also capturing the number of bits as an expression
    // note that u64s have been matched in the previous example and will not match on this one
    ([$field : ident, $field_ty : ty, u, $bits : expr, $($_ : tt,)*]; $my_ident_param : ident) => {
        println!("field XXX is XXX, which is unsigned and has {} bits", $bits); // TODO string stuff
        let mut something_of_the_same_type : $field_ty = $my_ident_param.$field;
        // this would fail during runtime on an_u64 because the example below sets it to u64::MAX
        something_of_the_same_type += 1;
        $my_ident_param.$field = something_of_the_same_type;
    };
    // catch an unknown field type and capture everything
    // this should be preferred to (_)
    ([$field : ident, $field_ty : ty, $($_ : tt,)*]; $my_ident_param : ident) => {

    };
    // absolutely anything else, here only present for debugging purposes
    //(_) => {};
}


fn main() {
    let mut my_data = MyData{
        some_text: "this is some text".to_string(),
        a_bool: true,
        an_u64: u64::MAX, // make adding a 1 crash this example
        an_u8: 123,
        an_i8 : -100,
        an_i16 : -1000,
        a_ff32: 420.69,
        a_ff64: 1337.69,
        more_text: "this is more text".to_string(),
        optional_thing: None,
    };

    // TODO: use macro here


    // the macro can of course also be used directly, as if called from the reflector
    // this is here for debugging purposes and to show internal workings
    // 
    for_each_field!([a_bool, bool, _,]; my_data);
    for_each_field!([some_text, String, _,]; my_data);
    // see if the u64 specific match catches on or leads to a runtime crash
    for_each_field!([an_u64, u64, u, 64, _,]; my_data);
    for_each_field!([an_u8, u8, u, 16, _,]; my_data);

    //fn_name();
}

