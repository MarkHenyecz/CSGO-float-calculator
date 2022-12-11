pub mod floatmerge;

fn main() {
    let files = floatmerge::read_files();
    let filtered = floatmerge::filter_files(files);

    let output = floatmerge::format_output(filtered).unwrap();
    let skins = floatmerge::output_to_skin(output);

    for skin in skins {
        println!("{}", skin.name);
        println!("{}", skin.float);
        println!("{}", skin.price);
    }
}
