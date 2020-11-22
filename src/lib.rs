#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn fallible() -> Result<()>;
    }
}

pub fn fallible() -> Result<(), std::string::FromUtf8Error> {
    let sparkle_heart = vec![240, 159, 146, 150];
    String::from_utf8(sparkle_heart)?;
    Ok(()) 
}
