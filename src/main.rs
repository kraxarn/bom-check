use walkdir::WalkDir;
use std::fs::read;
use std::env::args;

fn is_bom(data: &[u8]) -> bool {
    if data.len() < 3 {
        false
    } else {
        data[0] == 0xef && data[1] == 0xbb && data[2] == 0xbf
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        println!("usage:   bomcheck <filters> <directory>\nfilters: --utf8 --utf8-bom --not-utf8");
        return;
    }

    let utf8 = args.contains(&String::from("--utf8"));
    let utf8_bom = args.contains(&String::from("--utf8-bom"));
    let not_utf8 = args.contains(&String::from("--not-utf8"));

    for entry in WalkDir::new(args.last().unwrap()) {
        match entry {
            Ok(entry) => {
                if entry.path().is_dir() {
                    continue;
                }
                match read(entry.path()) {
                    Ok(file) => {
                        if is_utf8::libcore::is_utf8(file.as_slice()) {
                            if is_bom(file.as_slice()) {
                                if utf8_bom {
                                    println!("utf8-bom: {}", entry.path().display());
                                }
                            } else if utf8 {
                                println!("utf-8: {}", entry.path().display());
                            }
                        } else if not_utf8 {
                            println!("not-utf8: {}", entry.path().display());
                        }
                    },
                    Err(err) => println!("file error: {}", err)
                }
            },
            Err(err) => println!("walk error: {}", err)
        }
    }
}
