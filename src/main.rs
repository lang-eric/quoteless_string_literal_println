use rust_macro_experiments::quote_free_printer;

fn main() {
    // My expectation is that the below line of code will printer what is between the parens to
    // Stdout. Interestingly, in reality, a space is appended between the u and the '
    // when the word "you'd" is printed.
    quote_free_printer!(There is no conceivable reason why you'd ever need this to work.);
}