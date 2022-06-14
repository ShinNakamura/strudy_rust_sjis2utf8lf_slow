// 参考 https://stackoverflow.com/questions/64040851/how-can-i-read-a-non-utf8-file-line-by-line-in-rust
// ********** このコードは遅い ************
// 同じことをするなら https://github.com/ShinNakamura/study_rust_sjis2utf8lf のほうが高速
// とはいえ、Rust 学習では学びが多かったのでリポジトリに残しておく
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Write, BufWriter, BufReader, BufRead};
use std::env;
use std::path::PathBuf;

use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() -> MyResult<()> {
    let args: Vec<String> = env::args().collect();
    let filename = if args.len() > 1 {
        &args[1]
    } else {
        "-"
    };
    let file = open(filename)?;
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(file));
    for line in reader.lines() {
        writeln!(out, "{}", line?)?;
    }
    out.flush()?;
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => {
            let stdin = stdin();
            Ok(Box::new(BufReader::new(stdin.lock())))
        },
        _ => {
            let filepath = PathBuf::from(filename);
            Ok(Box::new(BufReader::new(File::open(filepath)?)))
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::error::Error;
    use assert_cmd::Command;
    type TestResult = Result<(), Box<dyn Error>>;
    #[test]
    fn test_arg() -> TestResult {
        let sjis_filepath = PathBuf::from(".").join("tests").join("sjis.txt");    
        let expected = "阿鼻叫喚 闊歩\n";
        Command::cargo_bin("sjis2utf8lf_slow")?
            .arg(&sjis_filepath)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
}

