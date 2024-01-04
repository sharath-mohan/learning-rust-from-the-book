
enum IpAdd{
    V4(u8,u8,u8,u8),
    V6(String)
}
fn main() {
    let home_addr = IpAdd::V6(String::from("::1"));
    let loop_back = IpAdd::V4(127, 0, 0, 1);

}
