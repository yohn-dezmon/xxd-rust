use std::fs::File;
use std::io::{self, Read};
use tar::Archive;

fn main() -> io::Result<()> {
    let file = File::open("files.tar").unwrap();

    // create a tar archive
    let mut archive = Archive::new(file);

    // iterate over each entry (file) in the archive
    for file in archive.entries()? {
        // unwrap
        let mut file = file?;
        let mut content = Vec::new();
        file.read_to_end(&mut content)?;
        // content is an array of decimal values
        // convert to hexadecimal and print to stdout
        let hex_string = hex::encode(&content);
        println!("{}", hex_string);

        // Convert content to slice of u16
        let content_u16: Vec<u16> = content.iter().map(|&x| x as u16).collect();
        // Decode content from UTF-16 Little Endian
        // skip first 3 characters with [2..] because they are a BOM
        // BOM = byte order mark  (indicates byte order and encoding of the text)
        let decoded_content = String::from_utf16_lossy(&content_u16[2..]);

        // Print the decoded content
        println!("{}", decoded_content);
    }
    Ok(())
}
