use clap::{app_from_crate, arg};

fn main() {
    let matches = cmd().get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let port: usize = matches
        .value_of_t("PORT")
        .expect("'PORT' is required and parsing will fail if its missing");
    println!("PORT = {}", port);
}

fn cmd() -> clap::Command<'static> {
    app_from_crate!().arg(
        arg!(<PORT>)
            .help("Network port to use")
            .validator(|s| s.parse::<usize>()),
    )
}

#[test]
fn verify_app() {
    cmd().debug_assert();
}
