# MacroRulesAutoreflector
a small compiletime reflection library for rust which creates a struct definition and wraps it into a <macro_rules!> macro which you supply to it.
the macro_rules! macro needs to be a tt muncher.

reflecting on a struct and suppying it your macro_rules! macro called make_print_fields! (omitted, see hello_world.rs in examples)
```
#[derive(Autoreflect)]
#[Autoreflect(make_print_fields)]
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
```
this creates the following automatically (you will ideally never see this):
```
make_print_fields!(@body {
    ([some_text, String]["some_text", "String"])
    ([a_bool, bool]["a_bool", "bool"])
    ([an_u64, u64]["an_u64", "u64"][u, 64])
    ([an_u8, u8]["an_u8", "u8"][u, 16])
    ([an_i8, i8]["an_i8", "i8"][i, 8])
    ([an_i16, i16]["an_i16", "i16"][i, 16])
    ([a_f32, f32]["a_f32", "f32"][f, 32])
    ([a_f64, f64]["a_f64", "f64"][f, 64])
    ([more_text, String]["more_text", "String"])
    ([optional_thing, Option<String>]["optional_thing", "Option<String>"])
});
```
this cannot deal with generics

TODO: remove syn/parse as it now works without it, OR make it capable of reading generics
