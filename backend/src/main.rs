fn main() {
    let config = match mathforge::Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
    if let Err(err) = mathforge::serve(config) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
