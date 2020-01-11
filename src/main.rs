mod bencoding;

fn main() {
    println!("{:?}", bencoding::decoder::decode("4:spam".to_string()));
}
