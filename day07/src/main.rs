use std::{fs, collections::HashMap};

// TODO Try to use a tree instead. But that is kinda hard in Rust I found out today .. 
#[derive(Debug)]
struct Filesystem {
    directories: Vec<String>,
    files: HashMap<String, (String, usize)>,
}

fn create_empty_filesystem() -> Filesystem {
    Filesystem { 
        directories: vec![],
        files: HashMap::new(),
    }
}

fn create_filesystem_from_commands(input: &str) -> Filesystem {
    let mut root = create_empty_filesystem();

    let mut path: Vec<String> = vec![];
    let mut line_iter = input.split("\n").peekable();

    while let Some(line) = line_iter.next(){    
        let cd_prefix = "$ cd ";
        if line.starts_with("$ cd /") {
            path.clear();
        } else if line.starts_with("$ cd ..") {
            path.pop();
        } else if line.starts_with(cd_prefix) {
            path.push(line[cd_prefix.len()..].to_string());
        } else if line == "$ ls" {
            loop {
                let abort = match line_iter.peek() {
                    None => true,
                    Some(next_line) => next_line.starts_with("$ "),
                };
                if abort {
                    break;
                }

                let result = line_iter.next().unwrap();

                if result.starts_with("dir ") {
                    let parts: Vec<_> = result.split(" ").collect();
                    let mut new_path = path.clone();
                    new_path.push(parts[1].to_string());
                    assert_directory_exists(&mut root, &new_path);
                } else {
                    let parts: Vec<_> = result.split(" ").collect();
                    insert_into_dir(&mut root, &path, parts[1], parts[0].parse::<usize>().unwrap());
                }
            }
        } else {
            panic!("Can not handle command");
        }
    }

    return root;
}

fn get_path_to_directory(path: &Vec<String>) -> String {
    let mut directory = path.join(&"/");
    directory.push_str("/");
    return directory;
}

fn assert_directory_exists(filesystem: &mut Filesystem, path: &Vec<String>) {
    let directory = get_path_to_directory(path);
    if !filesystem.directories.contains(&directory) {
        filesystem.directories.push(directory);
    }
}

fn insert_into_dir(filesystem: &mut Filesystem, path: &Vec<String>, file_name: &str, file_size: usize) {    
    assert_directory_exists(filesystem, path);

    let directory = get_path_to_directory(path);

    let mut full_path = directory.clone();
    full_path.push_str(file_name);

    filesystem.files.insert(full_path.to_string(), (file_name.to_string(), file_size));
}

fn find_directory_size(filesystem: &Filesystem, directory: &str) -> usize {
    return filesystem.files.iter()
        .filter(|(file_path, _)| file_path.starts_with(directory))
        .map(|(_file_path, (_file_name, file_size))| file_size)
        .sum();
}

fn main() {
    let contents = fs::read_to_string("assets/part-1.input").unwrap();
    let filesystem = create_filesystem_from_commands(&contents);

    // for directory in &filesystem.directories {
    //     println!("Dir {} is of size {}", directory, find_directory_size(&filesystem, &directory));
    // }

    let result: usize = filesystem.directories.iter()
        .map(|directory| find_directory_size(&filesystem, directory))
        .filter(|size| *size <= 100_000)
        .sum();

    println!("Sum of directories up to 100.000 is {:?}", result);
}
