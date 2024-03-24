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
        let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
        let asv = String::from("a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}");
        assert_eq!(usv_to_asv(&usv), asv);
    }

}
