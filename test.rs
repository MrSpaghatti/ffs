fn main() {
    let dest = "/foo/bar baz/qux.txt";
    let path = std::path::Path::new(dest);
    println!("{:?}", path.parent());
}
