#[cfg(test)]
mod memory {
    use lib::prelude::*;
    use std::{path::PathBuf, process::Command};

    const PROCESS_NAME: &str = "program.exe";

    fn spawn_external() {
        let process_path: PathBuf = std::env::current_dir()
            .unwrap()
            .join("external")
            .join(PROCESS_NAME);

        std::thread::spawn(move || {
            Command::new(process_path)
                .output()
                .expect("failed to spawn external program")
        });
    }

    #[test]
    fn open_process() {
        spawn_external();
        Memory::new(PROCESS_NAME).unwrap();
    }

    #[test]
    fn process_base_module() {
        spawn_external();
        let process = Memory::new(PROCESS_NAME).unwrap();
        assert_eq!(process.base_module.name, PROCESS_NAME);
    }

    #[test]
    fn read_write_process_memory() {
        spawn_external();
        let process = Memory::new(PROCESS_NAME).unwrap();
        let Ok(pointer) = process.calculate_pointer(
            unsafe { process.base_module.address.offset(0x241E0) },
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
    use lib::prelude::Timer;
    use std::time::Duration;

    #[test]
    #[allow(unused_assignments)]
    fn timer() {
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
}
