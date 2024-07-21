use anyhow::{Context, Result};
use chardet::{charset2encoding, detect};
use encoding_rs::Encoding;

pub fn transform_to_utf8(bytes: &[u8], encoding: Option<&str>) -> Result<String> {
    let detect_encoding = encoding.map_or_else(|| detect(bytes).0, |v| v.to_string());
    let encoding = charset2encoding(&detect_encoding);
    let ins = Encoding::for_label(encoding.as_bytes()).context("Failed to get encoding")?;
    let (cow, _, _) = ins.decode(bytes);
    let result = cow.to_string();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let file_path = std::path::Path::new("...");
        let filename = file_path.file_name().unwrap();
        let bytes = std::fs::read(file_path).unwrap();
        let result = transform_to_utf8(&bytes).unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let write = current_dir.join("./test").join(filename);
        let write_dir = write.parent().unwrap();
        // ensure dir
        if !write_dir.exists() {
            std::fs::create_dir_all(write_dir).unwrap();
        }
        std::fs::write(write, result).unwrap();
    }
}
