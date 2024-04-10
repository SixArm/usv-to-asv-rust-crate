use std::convert::AsRef;

pub mod examples; use examples::*;

pub fn usv_to_asv<
    S: AsRef<str> + Sized
>(
    usv: S,
) -> String {
    usv.as_ref()
    .replace("\u{001F}\u{001E}\u{001D}\u{001C}", "\u{001C}")
    .replace("\u{001F}\u{001E}\u{001D}", "\u{001D}")
    .replace("\u{001F}\u{001E}", "\u{001E}")
    .replace("\u{001F}", "\u{001F}") 
    .replace("␟␞␝␜", "\u{001C}")
    .replace("␟␞␝", "\u{001D}")
    .replace("␟␞", "\u{001E}")
    .replace("␟", "\u{001F}") 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols() {
        assert_eq!(usv_to_asv(usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS), EXAMPLE_ASV_FILES);
    }

    #[test]
    fn controls() {
        assert_eq!(usv_to_asv(usv::examples::EXAMPLE_FILES_STYLE_CONTROLS), EXAMPLE_ASV_FILES);
    }

}
