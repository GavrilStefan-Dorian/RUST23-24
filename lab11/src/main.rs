use std::fs::File;
use std::io::Write;
use std::io::Result;

struct MyWriter {
    file: File,
}

impl MyWriter {
    fn new(file_arg: File) -> Self {
        MyWriter{file: file_arg}
    }
}

impl Write for MyWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut bytes_parsed = 0;
        
        for byte in buf {
            match self.file.write(&[*byte, *byte]){
                Ok(2) => {
                    bytes_parsed+=1;
                }
                Ok(size) => {
                    return Ok(size);
                }
                Err(_) => {
                    return Ok(bytes_parsed);
                }
            }
        }
        Ok(bytes_parsed)
    }

    fn flush(&mut self) -> Result<()>{
        self.file.flush()
    }
}

fn main() -> Result<()> {
    let mut writer = MyWriter::new(File::create("a.txt")?);
    writer.write_all(b"abc")?;

    Ok(())
}

