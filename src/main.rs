use std::thread;
use std::time::Duration;
use std::io::Error;

pub mod utils;
pub mod what;
pub mod cool;
pub mod file;

fn main() -> Result<(), Error> {
    utils::logger::debug("Making wut");
    let wut = what::What(255, 255, 254);
    wut.debug();
    utils::logger::debug(wut);
    utils::logger::debug(wut.whatwhat());

    let thrd = thread::spawn(move || {
        utils::logger::debug("Inside thread");
        // New block context
        thread::sleep(Duration::from_millis(2000));
        let wut = what::What(255, 255, 254);
        wut.debug();

        file::FileDebug.open();
        utils::logger::debug("Thread finished");
    });

    utils::logger::debug("Waiting for thread to finish");
    thrd.join().unwrap();

    Ok(())
}
