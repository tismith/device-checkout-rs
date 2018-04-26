#![feature(plugin)]
#![plugin(rocket_codegen)]

//standard includes
extern crate failure; //this crate has macros, but this current program doesn't make use of them
#[macro_use]
extern crate log;
extern crate stderrlog;
#[macro_use]
extern crate clap;
extern crate rocket;

mod utils;

#[get("/")]
fn index() -> &'static str {
    trace!("default route called");
    "Hello, world!"
}

fn run(_config: &utils::types::Settings) -> Result<(), failure::Error> {
    trace!("Entry to top level run()");

    rocket::ignite().mount("/", routes![index]).launch();

    Ok(())
}

fn main() {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config);

    if let Err(ref e) = run(&config) {
        use failure::Fail;
        let mut fail: &Fail = e.cause();
        error!("{}", fail);

        while let Some(cause) = fail.cause() {
            error!("caused by: {}", cause);
            fail = cause;
        }
        std::process::exit(1);
    }
}
