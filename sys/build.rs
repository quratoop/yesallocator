use std::env;
use std::path::PathBuf;
use which::which;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[must_use]
fn get_root() -> Result<PathBuf> {
    match env::var_os("CARGO_MANIFEST_DIR") {
        Some(path) if path.is_empty() => Err("CARGO_MANIFEST_DIR is empty".into()),
        Some(path) if !PathBuf::from(&path).exists() => Err("CARGO_MANIFEST_DIR is not a valid path".into()),
        Some(path) if !PathBuf::from(&path).is_dir() => Err("CARGO_MANIFEST_DIR is not a directory".into()),
        Some(path) => Ok(PathBuf::from(path)),
        None => Err("CARGO_MANIFEST_DIR is not set".into()),
    }
}

#[must_use]
fn get_src() -> Result<PathBuf> {
    Ok(get_root()?.join("backend"))
}

#[must_use]
fn get_compiler() -> Result<PathBuf> {
    ["clang", "gcc", "cc"]
        .iter()
        .find_map(|&compiler| which(compiler).ok())
        .ok_or_else(|| "No suitable compiler found (clang, gcc, cc)".into())
}

#[must_use]
fn get_builder() -> Result<cc::Build> {
    let mut build = cc::Build::new();
    build.compiler(get_compiler()?);
    build.flag("-Wno-unused-parameter");
    build.file(get_src()?.join("yesalloc.c"));
    build.include(get_src()?);
    Ok(build)
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed={:?}", get_root()?);
    println!("cargo:rerun-if-changed={:?}", get_src()?.join("yesalloc.c"));
    println!("cargo:rerun-if-changed={:?}", get_src()?.join("yesalloc.h"));
    let builder = get_builder()?;
    builder.compile("yesalloc");
    println!("cargo:rustc-link-lib=static=yesalloc");
    Ok(())
}
