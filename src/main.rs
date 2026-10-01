use incus_isolation_adapter::{MAX_DOCUMENT_BYTES, parse_inventory, transform};
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Read, Write},
};

const SAMPLE: &[u8] = include_bytes!("../examples/incus-inventory.sample.json");

fn main() {
    if let Err(error) = run() {
        eprintln!("Incus isolation transformation failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let source = match arguments.as_slice() {
        [] => SAMPLE.to_vec(),
        [command] if command == "sample" => SAMPLE.to_vec(),
        [command, path] if command == "transform" => read_bounded(path)?,
        [command] if matches!(command.as_str(), "help" | "-h" | "--help") => {
            println!("usage: incus-isolation-adapter [sample|transform INVENTORY]");
            return Ok(());
        }
        _ => return Err("expected sample or transform INVENTORY".into()),
    };
    let output = transform(parse_inventory(&source)?);
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    serde_json::to_writer_pretty(&mut writer, &output)?;
    writeln!(writer)?;
    Ok(())
}

fn read_bounded(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open(path)?;
    if !file.metadata()?.file_type().is_file() {
        return Err("inventory must be a regular file".into());
    }
    let mut source = Vec::new();
    file.take(u64::try_from(MAX_DOCUMENT_BYTES + 1)?)
        .read_to_end(&mut source)?;
    if source.len() > MAX_DOCUMENT_BYTES {
        return Err("inventory exceeds the size limit".into());
    }
    Ok(source)
}
