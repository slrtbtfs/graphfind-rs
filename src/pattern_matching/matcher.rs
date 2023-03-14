/// The Matcher type stands for any function that evaluates, given an element
/// in the base graph with the type Weight, if the pattern graph accepts this element.
///
/// As an example, we may define a function that tests if a node matches a Rust pattern.
///
/// Recommended ways to create conditions are using either the lambda functions or the [crate::matcher] macro.
pub type Matcher<Weight> = dyn Fn(&Weight) -> bool;

/// Creates a `Matcher` function from a given pattern
///
/// The syntax is similar to the `std::matches` macro.
/// Calling matcher with no arguments will match anything.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate rustgql;
/// use rustgql::pattern_matching::*;
///
/// # // This line is hidden in the docs but required to pass the docstest, see https://users.rust-lang.org/t/how-to-use-a-macro-in-the-doc-test/3664/5?u=slrtbtfs
///
/// # fn main() {
///
/// enum Person {
///     Student {name: String},
///     Prof {},
/// }
///
/// let student = matcher!(Person::Student{..});
///
/// let even = matcher!(i if i % 2 == 0);
/// # }
#[macro_export]
macro_rules! matcher {
    () => {matcher!(_)};
    ($(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        |__weight__: &_|
        match __weight__ {
            $( $pattern )|+ $( if $guard )? => true,
            _ => false
        }
    };
}
