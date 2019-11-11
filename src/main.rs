#[macro_use]
extern crate error_chain;
extern crate walkdir;
extern crate dirs;

use walkdir::{DirEntry, WalkDir};
use std::path::PathBuf;

error_chain! {
    foreign_links {
        WalkDir(walkdir::Error);
        Io(std::io::Error);
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() -> Result<()> {
    // TODO: THIS GETS USER ROOT DIR
    let home = dirs::home_dir();
    let home_dir = home.unwrap();
    let taloom_dir: PathBuf = ["/Users/ixquitilis", "Documents/workRepos/taloom"].iter().collect();

    // /Users/ixquitilis/Documents/workRepos/taloom/just-hire/src/main/angular2/src/app/view-applicant-rating-results/view-applicant-rating-results.component.html
    // just-hire/src/main/angular2/src/app/view-applicant-rating-results/view-applicant-rating-results.component.html
    println!("{:?}", home_dir);

    // TODO: THIS IS FOR TESTING WITH HOME DIR ABOVE
    // let path = std::fs::read_link("page_with_scrolling_frame.html")?;

    // TODO: THIS BLOCK IS FOR MAPPING ALL HTML FILES
    for entry in WalkDir::new(taloom_dir)
            .follow_links(true)
            .into_iter()
            .filter_entry(|x| !is_hidden(x))
            .filter_map(|e| e.ok()) {

        let f_name = entry.file_name().to_string_lossy();
        // let f_name = entry.path().display();

        // println!("{}", f_name);

        if f_name.ends_with(".html") {
            println!("{}", f_name);
        }
    }

    // TODO: THIS BLOCK IS FOR READING THE FILE CONTENT
    // let content = std::fs::read_to_string("../../../Documents/workRepos/taloom/just-hire/src/main/angular2/src/index.html")
    //     .expect("could not read file");

    // for line in content.lines() {
    //     println!("{}", line);
    // }

    Ok(())
}
