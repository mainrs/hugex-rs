use hugex::prelude::*;

fn main() {
    hex();
    us_zip_code();
}

// (#(?:[0-9a-fA-F]{8})|#(?:[0-9a-fA-F]{3}){1,2})
fn hex() {
    let hex_chars = in_between('a'..='f')
        .and(in_between('A'..='F'))
        .and(in_between('0'..='9'))
        .as_character_class();

    let regex = is_char('#')
        .and(
            // color codes with 8 digits (contain alpha).
            hex_chars
                .clone()
                .repeated()
                .exactly(8)
                .grouped_non_capturing(),
        )
        .or(
            // Color codes with either three or six digits.
            is_char('#').and(
                hex_chars
                    .repeated()
                    .exactly(3)
                    .grouped()
                    .repeated()
                    .at_least(1)
                    .at_most(2),
            ),
        )
        .grouped()
        .build();

    println!("{regex}")
}

// \d{5}(?:[-\s]\d{4})?
fn us_zip_code() {
    let separator = is_char('-').and(whitespace()).as_character_class();
    let plus_four = separator.and(digit().repeated().exactly(4));

    let regex = digit()
        .repeated()
        .exactly(5)
        .and(plus_four.grouped_non_capturing().or_not())
        .build();
    println!("{regex}");
}
