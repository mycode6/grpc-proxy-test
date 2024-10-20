use failure::Error;

fn main() -> Result<(), Error> {
    tonic_build::configure()
        .file_descriptor_set_path("bin")
        .compile(
            &["proto/echo.proto"],
            &["googleapis", "proto"],
        )?;

    Ok(())
}