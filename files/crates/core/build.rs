// Migrations (../../migrations) and the service registry (../../services.json) are embedded at
// compile time; rebuild when they change.
fn main() {
    println!("cargo:rerun-if-changed=../../migrations");
    println!("cargo:rerun-if-changed=../../services.json");
}
