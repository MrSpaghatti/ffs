fn main() {
    let parts = vec!["cp", "file.txt", "/foo/bar baz/qux.txt"];
    let joined = shell_words::join(&parts);
    println!("{:?}", joined);
}
