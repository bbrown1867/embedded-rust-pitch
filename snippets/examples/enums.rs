#[derive(Debug)]
enum Error {}

fn read_sensor_data() -> Result<u8, Error> {
    Ok(0)
}

fn process_sensor_data() -> Result<(), Error> {
    let sensor_value = read_sensor_data()?;
    println!("Value = {:?}", sensor_value);
    Ok(())
}

fn main() {
    process_sensor_data().unwrap();
}
