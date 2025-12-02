use chrono::prelude::*;
use std::io::Write;

const SECRET: &str = include_str!("SECRET");

fn main() {
    let now = chrono::Utc::now().date_naive();
    let (year, day) = (now.year(), now.day());

    let input_folder = format!("input");
    let date_folder = format!("{:04}_{:02}", year, day);
    let input_date_folder = format!("{}/{}", input_folder, date_folder);

    let folder_path = std::path::Path::new(&input_date_folder);
    if !folder_path.exists() {
        std::fs::create_dir_all(folder_path).expect("Could not create input date folder");
    }

    let secret_first = &SECRET[0..5];
    let input_name_cache = format!("input_{}", secret_first);
    let input_file_path_cache = format!("{}/{}", input_date_folder, input_name_cache);
    let cache_file_path = std::path::Path::new(&input_file_path_cache);

    let cache_content = if !cache_file_path.exists() {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

        let response = ureq::get(&url)
            .set("Cookie", &format!("session={}", SECRET.trim()))
            .set(
                "User-Agent",
                "github.com/ErikWDev/aoc_rust by ErikWDev@gmail.com",
            )
            .call()
            .expect("Could not call adventofcode.com. Is the provided SECRET correct?");

        let content = response
            .into_string()
            .expect("Response content could not be read as string");

        let mut file = std::fs::File::create(cache_file_path).expect("Could not create input file");
        let _ = write!(file, "{}", content);

        content
    } else {
        std::fs::read_to_string(cache_file_path).expect("Could not read cache file")
    };

    let input_file_path = format!("{}/input", input_date_folder);
    let file_path = std::path::Path::new(&input_file_path);

    if file_path.exists() {
        let current_content = std::fs::read_to_string(file_path)
            .expect("Cache file exists but could not be read to string");

        if current_content == cache_content {
            return;
        }
    }

    let mut file = std::fs::File::create(file_path).expect("Could not create input file");
    let _ = write!(file, "{}", cache_content);
}
