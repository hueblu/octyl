fn main() {
    // delete program.log file if it exists
    std::fs::remove_file("log").unwrap_or_default();
}
