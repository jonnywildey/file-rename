use structopt::StructOpt;
use std::fs;
/// Recursively search for a pattern in file names and replace
#[derive(StructOpt, Clone)]
struct Cli {
    /// The pattern to look for
    #[structopt(long, short)]
    pattern: String,
    /// replace the pattern
    #[structopt(long, short)]
    replace: String,
    /// The path to the file to read
    #[structopt(long, parse(from_os_str))]
    path: std::path::PathBuf,
    /// Whether to skip renaming directories
    #[structopt(long, short)]
    omit_directories: bool,
    /// In test mode, files will not be renamed
    #[structopt(long, short)]
    test: bool,
}

type RenameList = Vec<(std::path::PathBuf, std::path::PathBuf)>;

fn main() {
    let args = Cli::from_args();
    let rename_list = rename_directory(args.clone(), 50);

    for (old_path, new_path) in rename_list {
        println!("{:?} -> {:?}", old_path.to_str(), new_path.to_str());
        if !args.test {
          fs::rename(old_path, new_path).expect("Error renaming file");
        }
   }
}

fn rename_directory(args: Cli, limit: u8) -> RenameList  {
 let mut rename_list: RenameList  =  vec![];
 // Get a list of all entries
 
 for entry in args.path.read_dir().expect("Path must be a valid directory") {
   if let Ok(entry) = entry {
     let name = entry.file_name().to_str().expect("No filename").to_string();
      if name.contains(&args.pattern) {
          let new_name = name.replace(&args.pattern, &args.replace);
          let old_path = entry.path();
          let new_path = entry.path().with_file_name(new_name);
          rename_list.push((old_path, new_path));
      }
   }
 }
rename_list
}