uniffi::include_scaffolding!("calculator");

pub fn welcome(name: String) -> String {
    format!("Welcome {name}, your calculator is ready")
}
