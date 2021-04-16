use std::fs;
use structopt::StructOpt;

/// Recursively search for a pattern in file names and replace
#[derive(StructOpt, Clone)]
struct FileRenameArgs {
    /// The pattern to look for
    #[structopt(long, short)]
    pattern: String,
    /// replace the pattern
    #[structopt(long, short)]
    replace: String,
    /// The path to the file to read
    #[structopt(long, parse(from_os_str))]
    path: std::path::PathBuf,
    /// In test mode, files will not be renamed
    #[structopt(long, short)]
    test: bool,
}

type RenameList = Vec<(std::path::PathBuf, std::path::PathBuf)>;

fn main() {
    let args = FileRenameArgs::from_args();
    let rename_list = rename_directory(args.clone(), 50);

    write_files(rename_list, args);
}

fn rename_directory(args: FileRenameArgs, limit: u8) -> RenameList {
    let mut rename_list: RenameList = vec![];
    // Get a list of all entries

    for entry in args
        .path
        .read_dir()
        .expect("Path must be a valid directory")
    {
        if let Ok(entry) = entry {
            let current_path = entry.path();
            let current_path_as_str = current_path.to_str().expect("Error reading path");

            if current_path.is_dir() {
                let new_args = FileRenameArgs {
                    path: current_path.clone(),
                    ..args.clone()
                };
                rename_list.append(&mut rename_directory(new_args, limit - 1));
            }
            if current_path_as_str.contains(&args.pattern) && current_path.is_file() {
                let new_path = std::path::Path::new(
                    &current_path_as_str.replace(&args.pattern, &args.replace),
                )
                .to_path_buf();
                rename_list.push((current_path.clone(), new_path));
            }
        }
    }
    rename_list
}

fn write_files(rename_list: RenameList, args: FileRenameArgs) {
    for (old_path, new_path) in rename_list {
        println!(
            "{} -> {}",
            old_path.to_str().expect("Invalid Path"),
            new_path.to_str().expect("Invalid Path")
        );
        if !args.test {
            let parent = new_path.parent();
            match parent {
                Some(result) => {
                    fs::create_dir_all(result).expect("Error creating directory");
                }
                None => (),
            }

            fs::rename(old_path, new_path).expect("Error renaming file");
        }
    }
}
