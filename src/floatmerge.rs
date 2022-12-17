use std::{fs::{self, ReadDir, File}, path::PathBuf, io::{Write}};


pub struct Skin {
    pub name: String,
    pub float: f64,
    pub price: String,
}

pub fn read_files() -> ReadDir {
    return fs::read_dir("./").unwrap()
}

pub fn filter_files(files: ReadDir) -> Vec<PathBuf> {
    let mut response = Vec::new();
    
    for file in files {
        let path = file.unwrap().path();
        if path.extension() != None && path.extension().unwrap() == "txt" {
            response.push(path)
        }
    }

    return response;
}

pub fn format_output(files: Vec<PathBuf>) -> std::io::Result<File> {
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

pub fn output_to_skin() -> Vec<Skin> {
    let mut response = Vec::new();

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
                let float = data.1.parse::<f64>().unwrap();
                skin.float = float;
            }
            if data.0 == 2 {
                skin.price = String::from(data.1);
            }
        }
        response.push(skin);
    }

    return response;
}