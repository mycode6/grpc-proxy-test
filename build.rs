use std::{env, path::PathBuf};

use failure::Error;

fn main() -> Result<(), Error> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("echo.bin"))
        .compile(
            &["proto/echo.proto"],
            &["googleapis", "proto"],
        )?;

    Ok(())
}