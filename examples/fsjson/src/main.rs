use std::{io,fs};
use std::io::{BufWriter,Write};
use std::fs::File;

trait JSONSerializable {
    fn to_json(&self) -> Vec<u8>;
}

trait JSONToFile: JSONSerializable {
    fn json_to_file(&self, s: fs::File) -> io::Result<()>;
}

struct MyType { val: u32 }

impl JSONSerializable for MyType {
    fn to_json(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for v in br#"{"type":"int", "value":"#.iter() {
            bytes.push(*v);
        }
        for v in format!("{}", self.val).bytes() {
            bytes.push(v);
        }
        bytes.push(b'}');
        bytes
    }
}

impl JSONToFile for MyType {
    fn json_to_file(&self, fd: fs::File) -> io::Result<()> {
        let mut writer = BufWriter::new(fd);
        writer.write(self.to_json().as_slice())?;
        Ok(())
    }
}

fn write_json<T: JSONToFile>(w: T, filepath: &str) -> io::Result<()> {
    let fd = File::create(filepath)?;
    w.json_to_file(fd)
}

fn main() -> io::Result<()> {
    write_json(MyType { val: 42u32 }, "example.json")
}
