extern crate chrono;
extern crate walkdir;

use std::io;
use std::fs;
use std::path::Path;
use std::time::*;
use chrono::prelude::*;
use walkdir::WalkDir;

//TODO make use of this function
/*fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)
            }
        }
    }
    Ok(())
}*/

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => { // unlikely but should be handled
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        },
    };
    Utc.timestamp(sec, nsec)
}

//Todo make passing parametr if needed
fn recursivly_walking_dir_old() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    let system_time = system_time_to_date_time(metadata.modified().unwrap());
                    let datetime: DateTime<Utc> = system_time.into();
                    println!(" {:?} {:?}", entry.path(), datetime );
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }
}

fn main() {
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        let system_time = entry.metadata().unwrap().modified().unwrap();
        let datetime: DateTime<Utc> = system_time.into();
        println!("{:?} {:?}",entry.path(), datetime);
    }
}
