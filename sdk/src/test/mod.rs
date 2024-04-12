// open ../../external/program.exe

#[cfg(test)]
mod memory {
    use lib::prelude::*;
    const PROCESS_NAME: &str = "program.exe";

    #[test]
    fn open_process() {
        Memory::new(PROCESS_NAME).unwrap();
    }

    #[test]
    fn process_base_module() {
        let process = Memory::new(PROCESS_NAME).unwrap();
        assert_eq!(process.base_module.name, PROCESS_NAME);
    }

    #[test]
    fn read_write_process_memory() {
        let process = Memory::new(PROCESS_NAME).unwrap();
        let Ok(pointer) = process.calculate_pointer(
            process.base_module.address + 0x241E0,
            &[0x18, 0x18, 0xc8, 0x28, 0x8ec],
        ) else {
            panic!("failed to calculate pointer");
        };

        if let Ok(data) = process.read::<i32>(pointer) {
            assert_eq!(data, 100);
        } else {
            panic!("failed to read process memory");
        };
    }
}

#[cfg(test)]
mod utils {
    use lib::prelude::*;

    #[test]
    fn stringify_vec_bytes() {
        let bytes = vec![0x36, 0x39];
        let string = stringify_bytes_u8(bytes);
        assert_eq!(string, String::from("69"));
    }
}

#[cfg(test)]
mod time {
    use lib::{prelude::Timer, time::Timers};
    use std::{collections::HashMap, time::Duration};

    #[derive(Hash, PartialEq, Eq)]
    enum Tags {
        One,
    }

    #[test]
    #[allow(unused_assignments)]
    fn timer_once() {
        let timer = Timer::default();
        let mut data = false;

        loop {
            if timer.once(Duration::from_secs(2)) {
                data = true;
                break;
            }
        }
        assert!(data);
    }

    #[test]
    fn timer_every() {
        let mut timer = Timer::default();
        let mut count = 0;

        loop {
            if timer.elapsed(Duration::from_secs(1)) {
                count += 1;
            }

            if count >= 5 {
                break;
            }
        }
        assert_eq!(count, 5);
    }
}
