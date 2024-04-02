use std::convert::AsRef;

pub fn usv_to_asv<
    S: AsRef<str> + Sized
>(
    usv: S,
) -> String {
    usv.as_ref()
    .replace("␟", "\u{001F}") 
    .replace("␞", "\u{001E}")
    .replace("␝", "\u{001D}")
    .replace("␜", "\u{001C}")
    .replace("␛", "\u{001B}")
    .replace("␄", "\u{0004}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(usv_to_asv(usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS), usv::examples::EXAMPLE_FILES_STYLE_CONTROLS);
    }

}
