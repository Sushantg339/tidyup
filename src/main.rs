use std::env;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

// take terminal input
fn term_input() -> PathBuf {
    let inp: Vec<String> = env::args().collect();
    let dir_name: PathBuf = if inp.len() < 1 || inp[2] == "." {
        env::current_dir().unwrap()
    } else {
        PathBuf::from(&inp[2])
    };
    return dir_name;
}

// get vector of all entries in the directory input
fn get_dir_entries(dir_path: &PathBuf) -> Vec<DirEntry> {
    let dir_content_result = fs::read_dir(dir_path);
    let dir_entries = match dir_content_result {
        Ok(entries) => {
            let mut entry_vec = Vec::new();
            for entry in entries {
                match entry {
                    Ok(val) => entry_vec.push(val),
                    Err(err) => println!("Error reading entry: {}", err),
                }
            }

            entry_vec
        }
        Err(err) => {
            println!("Error getting dir entries : {}", err);
            Vec::new()
        }
    };

    dir_entries
}

fn main() {
    let dir_path = term_input();
    let dir_entries = get_dir_entries(&dir_path);

    // Vectors of Dir Entries
    let mut imgs: Vec<DirEntry> = Vec::new();
    let mut docs = Vec::new();
    let mut vids = Vec::new();
    let mut aud = Vec::new();

    for dir_item in dir_entries {
        match dir_item.path().extension() {
            Some(extension) => {
                let ext = extension.to_string_lossy().to_string();
                if matches!(
                    ext.as_str(),
                    "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg"
                ) {
                    imgs.push(dir_item);
                } else if matches!(ext.as_str(), "docx" | "pdf" | "txt" | "xlsx" | "pptx") {
                    docs.push(dir_item);
                } else if matches!(ext.as_str(), "mp4" | "mkv" | "avi" | "mov" | "webm") {
                    vids.push(dir_item);
                } else if matches!(ext.as_str(), "mp3" | "wav" | "flac" | "aac" | "ogg") {
                    aud.push(dir_item);
                }
            }
            None => println!("{:?} is a directory", dir_item.path().file_name().unwrap()),
        };
    }

    // Path buffs for grouping files into similar file types
    let image: PathBuf;
    let document: PathBuf;
    let video: PathBuf;
    let audio: PathBuf;

    if imgs.len() > 0 {
        image = dir_path.join("Images");
        fs::create_dir_all(&image).unwrap();
        for img in imgs {
            let source = img.path();
            let destination = image.join(img.file_name());
            match fs::rename(&source, &destination) {
                Ok(()) => println!("Moved: {:?}", destination),
                Err(e) => println!("Error moving file: {}", e),
            }
        }
    }
    if docs.len() > 0 {
        document = dir_path.join("Documents");
        fs::create_dir_all(&document).unwrap();
        for doc in docs {
            let source = doc.path();
            let destination = document.join(doc.file_name());
            match fs::rename(&source, &destination) {
                Ok(()) => println!("Moved: {:?}", destination),
                Err(e) => println!("Error moving file: {}", e),
            }
        }
    }
    if vids.len() > 0 {
        video = dir_path.join("Videos");
        fs::create_dir_all(&video).unwrap();
        for vid in vids {
            let source = vid.path();
            let destination = video.join(vid.file_name());
            match fs::rename(&source, &destination) {
                Ok(()) => println!("Moved: {:?}", destination),
                Err(e) => println!("Error moving file: {}", e),
            }
        }
    }
    if aud.len() > 0 {
        audio = dir_path.join("Audios");
        fs::create_dir_all(&audio).unwrap();
        for a in aud {
            let source = a.path();
            let destination = audio.join(a.file_name());
            match fs::rename(&source, &destination) {
                Ok(()) => println!("Moved: {:?}", destination),
                Err(e) => println!("Error moving file: {}", e),
            }
        }
    }
    
}
