#[macro_use]
extern crate error_chain;
extern crate walkdir;
extern crate dirs;

// use walkdir::WalkDir;

error_chain! {
    foreign_links {
        WalkDir(walkdir::Error);
        Io(std::io::Error);
    }
}

fn main() -> Result<()> {
    // TODO: THIS BLOCK IS FOR MAPPING ALL HTML FILES
    // for entry in WalkDir::new("../../")
    //         .follow_links(true)
    //         .into_iter()
    //         .filter_map(|e| e.ok()) {

    //     let f_name = entry.file_name().to_string_lossy();

    //     if f_name.ends_with(".html") {
    //         println!("{}", f_name);
    //     }
    // }

    // TODO: THIS GETS USER ROOT DIR
    println!("{:#?}", dirs::home_dir());

    // TODO: THIS IS FOR TESTING WITH HOME DIR ABOVE
    // let path = std::fs::read_link("page_with_scrolling_frame.html")?;

    // TODO: THIS BLOCK IS FOR READING THE FILE CONTENT
    // let content = std::fs::read_to_string("../../../Documents/workRepos/taloom/just-hire/src/main/angular2/src/index.html")
    //     .expect("could not read file");

    // for line in content.lines() {
    //     println!("{}", line);
    // }

    Ok(())
}
