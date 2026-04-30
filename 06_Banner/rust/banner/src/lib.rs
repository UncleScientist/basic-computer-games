mod characters;

use std::collections::HashMap;

pub struct Banner {
    charmap: HashMap<char, [u16; characters::CHARACTER_WIDTH]>,
}

pub struct BannerIterator<'a> {
    // Constants
    banner_data: &'a Banner,
    text: Vec<char>,
    horiz: usize,
    vert: usize,
    pixel: Option<String>,

    // Iterator info
    letter: usize, // which letter of the text we're on
    column: usize, // current letter's column
    hpos: usize,   // which horizontal line we're on
}

impl Default for Banner {
    fn default() -> Self {
        Self::new()
    }
}

impl Banner {
    pub fn new() -> Self {
        let charmap = HashMap::from(characters::CHARACTER_MAP);
        Self { charmap }
    }

    pub fn banner<S: AsRef<str>, T: AsRef<str>>(
        &self,
        text: S,
        chars: Option<T>,
        horiz: usize,
        vert: usize,
    ) -> BannerIterator<'_> {
        let pixel = if let Some(ch) = chars {
            Some(ch.as_ref().into())
        } else {
            None
        };
        BannerIterator {
            banner_data: self,
            text: text.as_ref().chars().collect(),
            horiz,
            vert,
            pixel,
            letter: 0,
            column: 0,
            hpos: 0,
        }
    }

    fn generate_line(
        &self,
        ch: char,
        column: usize,
        vert_size: usize,
        pixel: Option<&str>,
    ) -> String {
        let pix = if let Some(pix) = pixel {
            pix.repeat(vert_size)
        } else {
            format!("{ch}").repeat(vert_size)
        };
        if let Some(data) = self.charmap.get(&ch) {
            let mut returnval = String::from("");
            if data[column] == 0 {
                return " ".repeat(characters::CHARACTER_HEIGHT * vert_size);
            } else {
                let bitmap = data[column] - 1;
                let mut bit = 10;
                while bit > 0 {
                    bit -= 1;
                    if (bitmap & (1 << bit)) != 0 {
                        returnval += &pix;
                    } else {
                        returnval += &" ".repeat(pix.len());
                    }
                }
            }
            returnval
        } else {
            ".".repeat(characters::CHARACTER_HEIGHT * vert_size)
        }
    }
}

impl Iterator for BannerIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hpos >= self.horiz {
            self.hpos = 0;
            self.column += 1;
        }
        if self.column == characters::CHARACTER_WIDTH {
            self.hpos += 1;
            return Some(String::from(""));
        } else if self.column > characters::CHARACTER_WIDTH {
            self.column = 0;
            self.letter += 1;
        }

        if self.letter >= self.text.len() {
            return None;
        }

        self.hpos += 1;
        Some(self.banner_data.generate_line(
            self.text[self.letter],
            self.column,
            self.vert,
            self.pixel.as_deref(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_space() {
        let banner = Banner::new();
        let mut text = banner.banner(" ", Some("#"), 3, 3);
        let line = text.next();
        let line = line.unwrap();
        println!("|{line}|");
        assert_eq!(characters::CHARACTER_HEIGHT * 3, line.len());
    }

    #[test]
    fn test_generate_letter_a() {
        const LETTER_A: [&str; characters::CHARACTER_WIDTH + 1] = [
            " ######   ",
            "    #  #  ",
            "    #   # ",
            "    #    #",
            "    #   # ",
            "    #  #  ",
            " ######   ",
            "",
        ];
        let banner = Banner::new();
        let text = banner.banner("A", Some("#"), 1, 1);
        for (index, line) in text.enumerate() {
            assert_eq!(LETTER_A[index], line.as_str());
        }
    }

    #[test]
    fn test_generate_vert_expansion() {
        const LETTER_A: [&str; characters::CHARACTER_WIDTH + 1] = [
            "  ############      ",
            "        ##    ##    ",
            "        ##      ##  ",
            "        ##        ##",
            "        ##      ##  ",
            "        ##    ##    ",
            "  ############      ",
            "",
        ];
        let banner = Banner::new();
        let text = banner.banner("A", Some("#"), 1, 2);
        for (index, line) in text.enumerate() {
            assert_eq!(LETTER_A[index], line.as_str());
        }
    }

    #[test]
    fn test_generate_horiz_expansion() {
        const LETTER_A: [&str; characters::CHARACTER_WIDTH * 2 + 2] = [
            " ######   ",
            " ######   ",
            "    #  #  ",
            "    #  #  ",
            "    #   # ",
            "    #   # ",
            "    #    #",
            "    #    #",
            "    #   # ",
            "    #   # ",
            "    #  #  ",
            "    #  #  ",
            " ######   ",
            " ######   ",
            "",
            "",
        ];
        let banner = Banner::new();
        let text = banner.banner("A", Some("#"), 2, 1);
        for (index, line) in text.enumerate() {
            assert_eq!(LETTER_A[index], line.as_str());
        }
    }
}
