use zip::write::FileOptions;
use std::fs::metadata;
use std::fs;

extern crate zip;

fn main() {
    std::process::exit(start());
}

fn start() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    let filename = &*args[1];
    let filepath = &*args[2];

    match zipwrite(filename, filepath) {
        Ok(_) => println!("File written to {}", filename),
        Err(e) => println!("Error: {:?}", e),
    }

    return 0;
}

fn zipwrite(filename: &str, filepath: &str) -> zip::result::ZipResult<()> {
    let logs = fs::read_dir(filepath).unwrap();
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);

    zip.add_directory(filepath, options)?;
    rwritedir(& mut zip, logs);
    zip.finish()?;
    Ok(())
}

fn rwritedir(zip: & mut zip::ZipWriter<std::fs::File>, logs: std::fs::ReadDir) {
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);
    for log in logs{
        let log = log.unwrap().path();
        let file = log.to_string_lossy().to_string();
        let md = metadata(&file).unwrap();

        if md.is_dir() {
            let logs = fs::read_dir(file).unwrap();
            rwritedir(zip, logs);
        }else{
            zip.start_file(file, options).expect("not file");
        }
    }
}