fn main() {
    let mut config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    config_max = None;

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No config!");
    }
}
