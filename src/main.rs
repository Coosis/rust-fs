use std::fs::DirEntry;
use std::io;
use std::cmp::max;
use std::process::exit;
use std::time::SystemTime;
use std::{fs, os::unix::fs::MetadataExt};
use std::path::PathBuf;

use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct FsCli {
    /// The root directory to start the search from
    #[arg(short = 'r', long = "root")]
    root: Option<PathBuf>,

    /// The file to inspect size of, only used if root is not provided
    #[arg(short = 'f', long = "file")]
    file: Option<PathBuf>,

    /// The maximum depth to dig to.
    /// Passing -1 means infinite, but beware larger values could result in long execution time.
    #[arg(short = 'd', long = "depth")]
    #[arg(default_value_t = 5)]
    depth: usize,

    /// If set to true, will only output filename and size, default is false
    #[arg(long = "clean")]
    #[arg(default_value_t = false)]
    clean: bool,

    /// If set to true, will reverse the rows shown, default is false
    #[arg(long = "reverse")]
    #[arg(default_value_t = false)]
    reverse: bool,
}

fn process_entry(entry: &DirEntry, dep: usize, mxdep: usize) -> Result<(String, String, u64), io::Error> {
    if dep > mxdep {
        return Ok((String::new(), String::new(), 0));
    }
    let path = entry.path();
    let entries = fs::read_dir(&path).unwrap();
    let name = path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let metadata = path.metadata()?;
    let modified = DateTime::<Local>::from(metadata.modified()?)
        .format("%Y-%m-%d %H:%M")
        .to_string();
    let sz = metadata.size();
    let mut szs: Vec<u64> = vec![];
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            match process_entry(&entry, dep+1, mxdep) {
                Ok((_, _, cur_sz)) => {
                    szs.push(cur_sz);
                    continue;
                }
                Err(e) => {
                    let name = entry.file_name().to_str().unwrap().to_string();
                    println!("Faulty when handling directory {}: {}, continuing...", name, e);
                    continue;
                }
            }
        }
        szs.push(metadata.size());
    }
    Ok((name, modified, sz + szs.iter().sum::<u64>()))
}

fn main() {
    let cli = FsCli::parse();
    let mut files: Vec<(String, String, u64)> = vec![];
    if cli.root.is_some() {
        let root = cli.root.unwrap();
        // we care about file size, modified time, and file name
        let entries = fs::read_dir(root).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let pathbuf = entry.path();
            let metadata = entry.metadata().unwrap();
            let modified = metadata
                .modified()
                .unwrap();
            let filename: String = pathbuf.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let datetime: String = DateTime::<Local>::from(modified)
                .format("%Y-%m-%d %H:%M")
                .to_string();
            if metadata.is_dir() {
                match process_entry(&entry, 0, cli.depth) {
                    Ok((name, modified, sz)) => {
                        files.push((name, modified, sz));
                    }
                    Err(e) => {
                        println!("An error occured when proccessing directory {}: {}, continuing...", filename, e);
                    }
                }
                continue;
            }
            let filesize = metadata.size();
            files.push((filename, datetime, filesize));
        }
    } else if cli.file.is_some() {
        let entry = cli.file.unwrap();
        let metadata = match fs::metadata(entry.to_path_buf())  {
            Ok(m) => m,
            Err(e) => {
                println!("An error occured when handling this file: {}", e);
                exit(1);
            }
        };
        let name = entry.file_name().unwrap().to_str().unwrap().to_string();
        if metadata.is_dir() {
            println!("Argument {} is not a file, consider using -r instead", name);
            exit(1);
        }
        let datetime: String = DateTime::<Local>::from(metadata.modified().unwrap())
            .format("%Y-%m-%d %H:%M")
            .to_string();
        let fs = metadata.size();
        files.push((name, datetime, fs));
    }

    let mut mx = 0;
    let mut mx2 = 0;
    files.iter().for_each(|(n, m, _)| {
        mx = max(mx, n.len());
        mx2 = max(mx2, m.len());
    });
    files.sort_by_key(|(_, _, s)| *s);
    if !cli.reverse {
        files.reverse();
    }
    if !cli.clean {
        println!("{:mx$} - {:mx2$} - {}", "File", "Last Modified", "Size");
        for _ in 0..mx+mx2+13 {
            print!("=");
        }
        println!();

        for (name, modify, fs) in files {
            println!("{:mx$} - {:mx2$} - {}", name, modify, fs);
        }
    }
    else {
        for (name, _, fs) in files {
            println!("{:mx$} - {}", name, fs);
        }
    }


    return;
}
