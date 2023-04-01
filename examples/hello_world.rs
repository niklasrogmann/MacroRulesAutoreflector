
extern crate macro_rules_autoreflector;
use std::fmt;

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
    a_f32: f32,
    a_f64: f64,
    more_text: String,
    optional_thing: Option<String>
}

/// the autoreflector will call this macro for each field
/// with a bunch of parameters to allow catching specific information
/// about fields
/// this example is intended for copy pasta
macro_rules! print_a_field {
    // the "$([$params : tt])*" skips any further field or meta information and is mandatory
    // to keep up with additional reflection data becoming available in future versions of this crate
    ([$field : ident, $field_ty : ty][$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("field {} : {} = {}", $field_string, $field_ty_string, $my_ident_param.$field);
    };
}

/// this example shows how to match / capture information
/// it also allows adding custom parameters, here: "$my_ident_param"
/// everything autoreflection related is contained in "[field,field_ty][field_string,field_ty_string][meta...];"
/// with custom parameters added after the ";"
macro_rules! match_field_information_and_do_things {
    // match bools and capture their names
    ([$field : ident, bool][$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => {
        // field_ty_string will always be "bool" here
        println!("{} is a {} and is {}", $field_string, $field_ty_string, $my_ident_param.$field);
    };
    // match whatever field is called some_text, regardless of its type
    ([some_text, $field_ty : ty] [$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => (
        // field_string will always be "some_text" here
        println!("there is a field called some_text of type {}", $field_ty_string);
    );
    // match u64s and print out their value
    // note that the type meta information "unsigned" and "64 bits"
    // do not need to be explicitly matched to do so
    ([$field : ident, u64][$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("u64 specific match of name {} has value: {}", $field_string, $my_ident_param.$field);
    };
    // match any other u* by capturing the field type and specifying the type u,
    // then further also capturing the number of bits as an expression
    // note that u64s have been matched in the previous example and will not match on this one
    ([$field : ident, $field_ty : ty][$field_string : expr, $field_ty_string : expr][u, $bits : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("field {} : {}, which is unsigned has {} bits", $field_string, $field_ty_string, $bits); // TODO string stuff
        let mut something_of_the_same_type : $field_ty = $my_ident_param.$field;
        // this would fail during runtime on an_u64 because the example below sets it to u64::MAX
        something_of_the_same_type += 1;
        $my_ident_param.$field = something_of_the_same_type;
    };
    // match any i*
    ([$field : ident, $field_ty : ty][$field_string : expr, $field_ty_string : expr][i, $bits : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("field {} : {}, which is signed has {} bits", $field_string, $field_ty_string, $bits); // TODO string stuff
    };
    // match any f*
    ([$field : ident, $field_ty : ty][$field_string : expr, $field_ty_string : expr][f, $bits : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("field {} : {}, which is a float has {} bits", $field_string, $field_ty_string, $bits); // TODO string stuff
    };
    // match and print any strings, but note that some_text has been cought explicitly before and will not match here
    ([$field : ident, String][$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("string: {}", $my_ident_param.$field); // TODO string stuff
    };
    // match a generic, for example an Option
    // note that this fails if T does not implement fmt::Display
    ([$field : ident, Option<$opt_ty : ty>][$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => {
        // opt_ty is unused in this example but shows how to match generics
        if let Some(o) = $my_ident_param.$field{
            println!("field {} is Some({})", $field_string, o.to_string());
        } else {
            println!("field {} is None", $field_string);
        }
    };
    
    // match anything else
    ([$field : ident, $field_ty : ty][$field_string : expr, $field_ty_string : expr] $([$params : tt])*; $my_ident_param : ident) => {
        println!("unmatched field: {} : {}", $field_string, $field_ty_string);
    };
}


fn main() {
    let mut my_data = MyData{
        some_text: "this is some text".to_string(),
        a_bool: true,
        an_u64: u64::MAX, // make adding a 1 crash this example
        an_u8: 123,
        an_i8 : -100,
        an_i16 : -1000,
        a_f32: 420.69,
        a_f64: 1337.69,
        more_text: "this is more text".to_string(),
        optional_thing: Some("something".to_string()),
    };

    // TODO: use macro here



    // use the macro directly, as if called from the reflector
    // this is here for debugging purposes and to show internal workings
    // 
    println!("#### print test:");
    print_a_field!([some_text, String]["some_text", "String"]; my_data);
    print_a_field!([a_bool, bool]["a_bool", "bool"]; my_data);
    // see if the u64 specific match catches on or leads to a runtime crash
    print_a_field!([an_u64, u64]["an_u64", "u64"][u, 64]; my_data);
    print_a_field!([an_u8, u8]["an_u8", "u8"][u, 16]; my_data);
    print_a_field!([an_i8, i8]["an_i8", "i8"][i, 8]; my_data);
    print_a_field!([an_i16, i16]["an_i16", "i16"][i, 16]; my_data);
    print_a_field!([a_f32, f32]["a_f32", "f32"][f, 32]; my_data);
    print_a_field!([a_f64, f64]["a_f64", "f64"][f, 64]; my_data);
    print_a_field!([more_text, String]["more_text", "String"]; my_data);
    // option does not impl fmt::Display and does not compile
    //print_each_field!([optional_thing, Option<String>]["optional_thing", "Option<String>"]; my_data);

    println!("#### matching test:");
    match_field_information_and_do_things!([some_text, String]["some_text", "String"]; my_data);
    match_field_information_and_do_things!([a_bool, bool]["a_bool", "bool"]; my_data);
    // see if the u64 specific match catches on or leads to a runtime crash
    match_field_information_and_do_things!([an_u64, u64]["an_u64", "u64"][u, 64]; my_data);
    match_field_information_and_do_things!([an_u8, u8]["an_u8", "u8"][u, 16]; my_data);
    match_field_information_and_do_things!([an_i8, i8]["an_i8", "i8"][i, 8]; my_data);
    match_field_information_and_do_things!([an_i16, i16]["an_i16", "i16"][i, 16]; my_data);
    match_field_information_and_do_things!([a_f32, f32]["a_f32", "f32"][f, 32]; my_data);
    match_field_information_and_do_things!([a_f64, f64]["a_f64", "f64"][f, 64]; my_data);
    match_field_information_and_do_things!([more_text, String]["more_text", "String"]; my_data);
    match_field_information_and_do_things!([optional_thing, Option<String>]["optional_thing", "Option<String>"]; my_data);
    
    //fn_name();
}

