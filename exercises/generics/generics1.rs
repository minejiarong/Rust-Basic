// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut shopping_list: Vec<&dyn std::fmt::Debug> = Vec::new();
    shopping_list.push(&&"milk");
}
