fn main() {
    let script = "cp file.txt /foo/bar baz/qux.txt";
    let parts = shell_words::split(script).unwrap();
    println!("{:?}", parts);
}
