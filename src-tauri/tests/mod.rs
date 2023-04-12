use jwalk::WalkDir;

#[test]
pub fn test_walkdir() {
    let mut walkdir = WalkDir::new("../testing").into_iter();

    while let Some(entry) = walkdir.next() {
        println!("{:?}", entry);
    }
}
