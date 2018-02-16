use super::*;

#[test]
fn test_get_file_content() {
    let result = get_file_content("test/file.txt".to_string())
        .unwrap();

    assert_eq!("foo\nbar\n", result);
}

#[test]
#[should_panic]
fn test_get_file_content_error() {
    let result = get_file_content("test/non_existing_file".to_string())
        .unwrap();
}

#[test]
fn test_concat() {
    let file_names = vec!["test/file.txt", "test/another_file.txt"];
    let result = concat(&file_names)
        .unwrap();

    assert_eq!("foo\nbar\nbaz\nqewxbirr\n", result);
}
