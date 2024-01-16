fn main() {
    lalrpop::Configuration::new()
        .process_dir("src/parser/")
        .unwrap();
}
