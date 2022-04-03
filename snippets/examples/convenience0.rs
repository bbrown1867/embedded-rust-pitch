fn get_memory_address() -> Option<u32> {
    Some(0x4000_0000)
}

fn main() {
    match get_memory_address() {
        Some(address) => println!("Address = {:X}", address),
        None => panic!("Uh oh"),
    }
}
