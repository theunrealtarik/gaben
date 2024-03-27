pub fn stringify_bytes_u8<T>(bytes: T) -> String
where
    T: IntoIterator,
    T::Item: Into<u8>,
{
    String::from_utf8(
        bytes
            .into_iter()
            .map(|i| {
                let i: u8 = i.into();
                i
            })
            .filter(|&i| i != 0)
            .collect::<Vec<u8>>(),
    )
    .unwrap_or_else(|_| String::new())
}

pub fn clear_screen() {
    clearscreen::clear().unwrap();
}
