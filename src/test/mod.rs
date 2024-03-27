#[cfg(test)]
mod tests {
    use lib::prelude::*;

    static PROCESS_NAME: &str = "program.exe";
    static ADDRESS: usize = 0x39fd0f6ac;

    // #[test]
    // fn open_process() {
    //     Memory::new(PROCESS_NAME);
    // }
    //
    // #[test]
    // fn read_process_memory() {
    //     let mem = Memory::new(PROCESS_NAME).unwrap();
    //     assert_eq!(mem.read::<i32>(ADDRESS), Ok(100));
    // }
    //
    // #[test]
    // fn write_process_memory() {
    //     let mem = Memory::new(PROCESS_NAME).unwrap();
    //
    //     let data = 200;
    //     let bytes = mem.write::<i32>(ADDRESS, data);
    //     assert_eq!(bytes, Ok(std::mem::size_of_val(&data)));
    // }

    #[test]
    fn stringify_vec_bytes() {
        let bytes = vec![0x36, 0x39];
        let string = stringify_bytes_u8(bytes);
        assert_eq!(string, String::from("69"));
    }
}
