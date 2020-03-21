#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
// modules are defined with mod keyword and it's name
// modules can have other modules  as well as structs, enums, constants, traits or functions
// You can organize code by splitting it into multiple modules

// Module System
// Packages - A Cargo feature that lets you build, test, and share crates
// Crates - A tree of modules that produces a library or executable
// Modules and use - Let you control the organization, scope and privacy of paths
// Paths - A way of naming an item, such as a struct, function, or module

// A crate is a binary or library
// The crate root is a source file that the Rust compiler starts from

// A package is one or more crates that provide a set of functionality
// A package contains a Cargo.toml file that describes how to build those crates

// A package must contain zero or one library crates, and no more
// It can contain as many binary crates as you'd like
// It must contain at least one crate (binary or library)

// Cargo follows a convention that src/main.rs is the crate root of a binary crate
// Cargo also knows that a package directory containing src/lib.rs is the crate root of a library crate

// Cargo passes the crate root files to rustc to build the library/binary

// Packages that only contain src/main.rs is a binary crate
// If a package contains src/main.rs and src/lib.rs it has two crates
//  a library and a binary crate (both have the same name as the package)
// Packages can have multiple binary crates by placing files in the src/bin directory
//  each file will be a separate binary crate

// the use keyword brings use keyword to bring a path into scope
// the pub keyword makes items public

// modules let us organize code within a crate into groups for readability and easy reuse
// modules also control the privacy of items (public/private)

// in order to find an item in the module tree we use a "path"
// paths can take two forms
// absolute path starts from the crate rot by using a crate name or a literal "crate"
// a relative path starts from the current module and uses self, super or an identifier in the current module
// both paths are follow by one or more identifiers separated by double colons (::)

// Glob operator (*) brings all public items into the current scope
use std::collections::*;

// using a semicolon after instead of a block tells Rust to load the contents of the module from another file with the same name as the module
mod other_file;
// "use" brings the "other_file" module into the scope
use crate::other_file::stuff;

mod front_of_house {
    // marking hosting as public exposes the hosting path
    pub mod hosting {
        // without pub this is still private from parent module
        pub fn add_to_wait_list() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    // it is also possible to make enums and structs public
    pub struct Breakfast {
        pub toast: String,      // each field has to be made public to access
        seasonal_fruit: String, // private outside of back_of_house module
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), // can be accessed here because it is in the same module
            }
        }
    }

    // if enumerators are made public than so are all the variants
    pub enum Appetizer {
        Soup, // no need for pub tags on variants if enum is pub
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // super goes to parent module (similar to .. in file systems)
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// use is used to bring a path into scope (can be relative (this is))
use front_of_house::hosting;
// use can be extended out to a full function
use front_of_house::hosting::add_to_wait_list;

// you can also provide new names with the as keyword
pub use front_of_house::hosting::add_to_wait_list as atwl;
// adding pub to use is called re-exporting because it allows other scopes to bring it into their scope

// the function is a part of library crate's public API so it's marked with "pub" keyword
pub fn eat_at_restaurant() {
    // because eat_at_restaurant and front_of_house are both in the same module front_of_house doesn't have to be public
    // Absolute Path Example
    crate::front_of_house::hosting::add_to_wait_list();
    // Relative Path Example
    front_of_house::hosting::add_to_wait_list();

    // use now makes full paths irrelevant
    hosting::add_to_wait_list();

    // now because use brought the function into scope hosting is no longer needed
    add_to_wait_list();

    // as keyword function in use declaration
    atwl();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

fn main() {}

// Module Tree:
// crate
// └── front_of_house
// ├── hosting
// │   ├── add_to_wait_list
// │   └── seat_at_table
// └── serving
//     ├── take_order
//     ├── serve_order
//     └── take_payment
