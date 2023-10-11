pub mod floatmerge;
use std::{io::{self, Write}, fs::File};
use itertools::Itertools;
use futures::future::join_all;

use floatmerge::Skin;

#[tokio::main]
async fn main() -> io::Result<()> {
    let files = floatmerge::read_files();
    let filtered = floatmerge::filter_files(files);

    let _ = floatmerge::format_output(filtered);
    let skins = floatmerge::output_to_skin();

    println!("File combination finished!");
    println!("Press Enter to continue, type Q to Save & Quit");

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;

    if user_input.replace("\n", "").replace("\r", "").to_lowercase() == "q" {
        println!("File saved as floats.csv");
        return Ok(());
    }

    println!("Please enter the skin float that you want:");
    let stdin = io::stdin();
    stdin.read_line(&mut user_input)?;
    let float_avrage = calculate_float_avrage(user_input);

    println!("Needed float average is: {}", float_avrage.0);

    generate_combinations(skins, float_avrage.0, float_avrage.1, float_avrage.2).await?;

    Ok(())
}

fn calculate_float_avrage(input: String) -> (f64, f64, f64) {
    let min_float = 0.06; // minimum float from range
    let max_float = 0.80; // maximum float from range
    let float_range = max_float - min_float; // float range of the available skin floats (eg. 0.00-0.99)
    let input_value = input.replace("\n", "").replace("\r", "").parse::<f64>().unwrap();

    return ((-min_float + input_value) / float_range, float_range, min_float); // calculates the float average from the given wanted float and the float range
}

struct CombinationResult<'a> {
    pub difference: f64,
    pub skins_avrage: f64,
    pub combination: Vec<&'a Skin>,
}

async fn generate_combinations(skins: Vec<Skin>, float_avrage: f64, float_range: f64, min_float: f64) -> io::Result<()> {
    let output_file_name = "combinations.csv";
    let output_file = File::create(output_file_name)?;
    output_file.set_len(0)?;

    let comb = skins.leak().iter().combinations(10);

    let mut last_diff = 100.0; // default last float avg difference

    let mut count: i32 = 0;
    let mut insert_id = 0;

    let worker_count = 10;
    let mut handles = Vec::new();

    for skin_comb in comb {
        count += 1;

        let handle = tokio::spawn(async move {
            let mut comb_total = 0.0;
            for skin in skin_comb.clone() {
                comb_total += skin.float;
            }

            let skins_avrage = comb_total / skin_comb.iter().count() as f64;

            // spawn(print_count(count));

            let mut difference = float_avrage - skins_avrage;

            if difference < 0.0 {
                difference = difference * -1.0;
            }

            return CombinationResult{
                difference: difference,
                combination: skin_comb,
                skins_avrage: skins_avrage
            };
        });

        handles.push(handle);
        if handles.len() >= worker_count {
            let results = join_all(handles).await;
            for handle in results {
                let result: CombinationResult = handle?;

                if last_diff > result.difference {
                    last_diff = result.difference;
                    let new_float = float_range * result.skins_avrage + min_float; // calculates the possible skin float from the float average of [n]th float combination
                
                    let mut futureskin_comb = vec![];
    
                    for skin in result.combination.clone() {
                        futureskin_comb.push(Skin{
                            name: skin.name.clone(),
                            float: skin.float,
                            price: skin.price.clone()
                        });
                    }
    
                    insert_id += 1;
                    write_to_file(output_file.try_clone()?, futureskin_comb, result.difference, count, insert_id, float_avrage, result.skins_avrage, new_float);
                }
    
                if float_avrage == result.skins_avrage {
                    last_diff = 100.0;
    
                    println!("Exact match found! If you want to continue press Enter, if not type q.");
    
                    let mut user_input = String::new();
                    let stdin = io::stdin();
                    let _ = stdin.read_line(&mut user_input);
    
                    if user_input.replace("\n", "").replace("\r", "").to_lowercase() == "q" {
                        break;
                    }
                }
            }
            handles = Vec::new();
        }
    }
    println!("Combinations checked: {}", count);
    println!("All combinations found in combinations.csv");

    Ok(())
}

fn write_to_file(mut output_file: File, skin_comb: Vec<Skin>, difference: f64, count: i32, insert_id: i32, wanted: f64, found: f64, possible: f64) {
    println!("New closer one found: {}", difference);
    println!("Calculation ID: {}", count);

    let _ = output_file.write_all("ID: {id}/{count}\n".replace("{id}", &insert_id.to_string()).replace("{count}", &count.to_string()).as_bytes());
    let _ = output_file.write_all("New closest average found: '{float}'\n".replace("{float}", &found.to_string()).as_bytes());
    let _ = output_file.write_all("Wanted float average: '{float}'\n".replace("{float}", &wanted.to_string()).as_bytes());
    let _ = output_file.write_all("Float average difference: '{float}'\n".replace("{float}", &difference.to_string()).as_bytes());
    let _ = output_file.write_all("Possible float from new closest average: '{float}'\n".replace("{float}", &possible.to_string()).as_bytes());
    let _ = output_file.write_all("Combination:\n".as_bytes());
    
    let mut price: f32 = 0.0; 
    let mut combination_string = String::from("(");
    for skin in skin_comb {
        combination_string.push_str(&"[{float}],".replace("{float}", &skin.float.to_string()));
        let price_modification = skin.price.replace("€", "").parse::<f32>();
        price += match price_modification {
            Ok(value) => value,
            Err(_) => 0.0
        };
    }
    combination_string.remove(combination_string.len()-1);
    combination_string.push_str(")\n");

    let _ = output_file.write_all(combination_string.as_bytes());
    let _ = output_file.write_all("Price of treade up: {price} €\n\n".replace("{price}", &price.to_string()).as_bytes());
}