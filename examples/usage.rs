use ubuntu_version::*;

fn main() {
    if let Some(ver) = ubuntu_version() {
        println!("{:?}", ver);
    } else {
        println!("This environment is not Ubuntu");
    }
}
