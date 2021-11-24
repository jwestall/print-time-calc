pub mod ptime {
    use std::io;

    pub struct PTime {
        pub hour: u8,
        pub min: u8,
        pub am: bool
    }

    impl PTime {
        pub fn get_ptime(time_name: String) -> PTime {
            let mut input = String::new();
            let mut output: [u8; 2] = [0, 0];
            let mut i = 0;

            println!("Enter the {} time as HH:MM below:", time_name);
            io::stdin().read_line(&mut input).expect("Error reading from stdin!");

            for s in input.split(":") {
                output[i] = s.trim().parse().expect("Error parsing input to u32!");
                i += 1;
            }

            PTime {
                hour: output[0],
                min: output[1],
                am: false
            }
        }
    }
}