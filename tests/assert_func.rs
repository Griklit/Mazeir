use std::fmt::Debug;
use std::io::Read;
use std::path::Path;

pub fn assert_files_eq<T: AsRef<Path> + Debug>(file1: T, file2: T) {
    let metadata1 = std::fs::metadata(&file1).expect("Unable to get metadata");
    let metadata2 = std::fs::metadata(&file2).expect("Unable to get metadata");
    assert_eq!(metadata1.len(), metadata2.len(), "Files {:?} and {:?} do not have the same size", file1, file2);

    let mut file1 = std::fs::File::open(&file1).expect("Unable to open file");
    let mut file2 = std::fs::File::open(&file2).expect("Unable to open file");

    let mut buffer1 = [0; 1024];
    let mut buffer2 = [0; 1024];

    loop {
        match (file1.read(&mut buffer1), file2.read(&mut buffer2)) {
            (Ok(n1), Ok(n2)) if n1 == n2 && n1 == 0 => break,
            (Ok(n1), Ok(n2)) if n1 == n2 => assert_eq!(buffer1[..n1], buffer2[..n2], "Files {:?} and {:?} are not equal", file1, file2),
            _ => panic!("Files {:?} and {:?} are not equal", file1, file2),
        }
    }
}