// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

mod sausage_factory {
    // module declaration not giveing public access to everything inside the module
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        // grant access to the outside world
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
