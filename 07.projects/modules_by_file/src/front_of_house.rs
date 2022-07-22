//this is similar to pythos __init__.py files but it contains the declaration to the module of the same name.
// once a module is declared in here it can be referenced all throughout the project, for example in the lib.rs file.
pub mod hosting;
pub mod serving;
// pub mod cooking; -> this is not accepted cuz cooking is in back_of_house