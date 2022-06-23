mod functions;
mod variables_and_mut;
mod data_types;

fn main() {
    variables_and_mut::mutability();
    variables_and_mut::constants(&5);
    variables_and_mut::shadowing_and_scopes();

    data_types::all();
}

