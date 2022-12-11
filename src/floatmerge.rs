use std::{fs::{self, ReadDir, File}, path::PathBuf, collections::LinkedList, io::{Write, Read}, string};


pub struct Skin {
    pub name: String,
    pub float: f32,
    pub price: String,
}

pub fn read_files() -> ReadDir {
    return fs::read_dir("./").unwrap()
}

pub fn filter_files(files: ReadDir) -> LinkedList<PathBuf> {
    let mut response = LinkedList::new();
    
    for file in files {
        let path = file.unwrap().path();
        if path.extension() != None && path.extension().unwrap() == "txt" {
            response.push_back(path)
        }
    }

    return response;
}

pub fn format_output(files: LinkedList<PathBuf>) -> std::io::Result<File> {
    let mut output_file = File::create("output.csv")?;
    output_file.set_len(0)?;

    for file_name in files {
        let data = fs::read_to_string(file_name).expect("Unable to read file");
        let data_array = data.split("\n");

        for line_data in data_array.clone().enumerate() {
            let line = line_data.1;
            if 
                line.contains("|") || 
                line.contains("Float: ") 
            {
                let mut final_line = line.replace("Float: ", "").trim().to_owned();
                final_line.push_str(",");
                let _ = output_file.write_all(final_line.as_bytes());
            }
            
            if line.contains("€")  {
                let mut final_line = line.replace(",", ".").trim().to_owned();
                final_line.push_str("\n");
                let _ = output_file.write_all(final_line.as_bytes());
            }
        }
    }

    Ok(output_file)
}

fn rem(path: &str) {
    use std::io::ErrorKind;
  
    let message = match std::fs::remove_file(path) {
      Ok(()) => "ok",
      Err(e) if e.kind() == ErrorKind::NotFound => "not found",
      Err(e) => "other",
    };
    println!("{message}");
}

pub fn output_to_skin(mut file: File) -> LinkedList<Skin> {
    let mut response = LinkedList::new();

    let data = fs::read_to_string("output.csv").expect("Unable to read file");

    let data_array = data.split("\n");
    let skin_count = data_array.clone().count();

    println!("Skin count: {}", skin_count);

    for line in data_array.clone().enumerate() {
        let skin_data = String::from(line.1);
        let skin_data_split = skin_data.split(",");

        let mut skin = Skin {
            name: String::from(""),
            float: 0.0,
            price: String::from(""),
        };

        for data in skin_data_split.enumerate() {
            if data.0 == 0 {
                skin.name = String::from(data.1);
            }
            if data.0 == 1 {
                let float = data.1.parse::<f32>().unwrap();
                skin.float = float;
            }
            if data.0 == 2 {
                skin.price = String::from(data.1);
            }
        }
        response.push_back(skin);
    }

    return response;
}