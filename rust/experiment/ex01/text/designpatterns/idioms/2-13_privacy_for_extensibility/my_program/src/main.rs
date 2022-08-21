use my_lib;
fn print_matched_variants(s: my_lib::abc::S) {
    // Because S is `#[non_exhaustive]`, it cannot be named here and
    // we must use `..` in the pattern.
    let my_lib::abc::S { foo: _, .. } = s;

    let some_enum = my_lib::abc::AdmitMoreVariants::VariantA;
    match some_enum {
        my_lib::abc::AdmitMoreVariants::VariantA => println!("it's an A"),
        my_lib::abc::AdmitMoreVariants::VariantB => println!("it's a b"),

        // .. required because this variant is non-exhaustive as well
        my_lib::abc::AdmitMoreVariants::VariantC { a, .. } => println!("it's a c"),
        // The wildcard match is required because more variants may be
        // added in the future
        _ => println!("it's a new variant"),
    }
}

fn main() {}
