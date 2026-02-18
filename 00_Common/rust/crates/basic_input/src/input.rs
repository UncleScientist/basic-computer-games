use std::{io::BufRead, io::StdoutLock, io::Write, str::FromStr};

struct IoLayer {
    output: Box<dyn TextOutput>,
    input: Box<dyn TextInput>,
}

pub fn input<F: FromStr, P: AsRef<str>>(prompt: P) -> F {
    let mut io = IoLayer::get_io();
    io.get_single_value(prompt)
}

pub fn input2<F: FromStr, G: FromStr, P: AsRef<str>>(prompt: P) -> (F, G) {
    let mut io = IoLayer::get_io();
    io.get_two_values(prompt)
}

pub fn input3<F: FromStr, G: FromStr, H: FromStr, P: AsRef<str>>(prompt: P) -> (F, G, H) {
    let mut io = IoLayer::get_io();
    io.get_three_values(prompt)
}

pub fn input4<F: FromStr, G: FromStr, H: FromStr, I: FromStr, P: AsRef<str>>(
    prompt: P,
) -> (F, G, H, I) {
    let mut io = IoLayer::get_io();
    io.get_four_values(prompt)
}

impl IoLayer {
    fn get_io() -> Self {
        Self {
            output: StdoutLayer::new(),
            input: StdinLayer::new(),
        }
    }

    fn get_single_value<F: FromStr, P: AsRef<str>>(&mut self, prompt: P) -> F {
        loop {
            self.output.write(format!("{}? ", prompt.as_ref()).as_str());
            let response = self.input.read();
            let values = crate::parse::parse(&response);
            if let Ok(result) = values[0].parse::<F>() {
                return result;
            }
            self.output.write("?Re-enter\n");
        }
    }

    fn get_two_values<F: FromStr, G: FromStr, P: AsRef<str>>(&mut self, prompt: P) -> (F, G) {
        let mut first: Option<F>;
        let second: Option<G>;

        loop {
            self.output.write(format!("{}? ", prompt.as_ref()).as_str());
            let response = self.input.read();
            let values = crate::parse::parse(&response);
            if let Ok(result) = values[0].parse::<F>() {
                first = Some(result);
            } else {
                self.output.write("?Re-enter");
                continue;
            }
            if let Ok(result) = values[1].parse::<G>() {
                second = Some(result);
                break;
            } else {
                self.output.write("?Re-enter");
            }
        }

        (first.unwrap(), second.unwrap())
    }

    fn get_three_values<F: FromStr, G: FromStr, H: FromStr, P: AsRef<str>>(
        &mut self,
        prompt: P,
    ) -> (F, G, H) {
        let mut first: Option<F>;
        let mut second: Option<G>;
        let third: Option<H>;

        loop {
            self.output.write(format!("{}? ", prompt.as_ref()).as_str());
            let response = self.input.read();
            let values = crate::parse::parse(&response);
            if let Ok(result) = values[0].parse::<F>() {
                first = Some(result);
            } else {
                self.output.write("?Re-enter");
                continue;
            }
            if let Ok(result) = values[1].parse::<G>() {
                second = Some(result);
            } else {
                self.output.write("?Re-enter");
                continue;
            }
            if let Ok(result) = values[2].parse::<H>() {
                third = Some(result);
                break;
            } else {
                self.output.write("?Re-enter");
            }
        }

        (first.unwrap(), second.unwrap(), third.unwrap())
    }

    fn get_four_values<F: FromStr, G: FromStr, H: FromStr, I: FromStr, P: AsRef<str>>(
        &mut self,
        prompt: P,
    ) -> (F, G, H, I) {
        let mut first: Option<F>;
        let mut second: Option<G>;
        let mut third: Option<H>;
        let fourth: Option<I>;

        loop {
            self.output.write(format!("{}? ", prompt.as_ref()).as_str());
            let response = self.input.read();
            let values = crate::parse::parse(&response);
            if let Ok(result) = values[0].parse::<F>() {
                first = Some(result);
            } else {
                self.output.write("?Re-enter");
                continue;
            }
            if let Ok(result) = values[1].parse::<G>() {
                second = Some(result);
            } else {
                self.output.write("?Re-enter");
                continue;
            }
            if let Ok(result) = values[2].parse::<H>() {
                third = Some(result);
            } else {
                self.output.write("?Re-enter");
                continue;
            }
            if let Ok(result) = values[3].parse::<I>() {
                fourth = Some(result);
                break;
            } else {
                self.output.write("?Re-enter");
            }
        }

        (
            first.unwrap(),
            second.unwrap(),
            third.unwrap(),
            fourth.unwrap(),
        )
    }

    #[cfg(test)]
    fn test() -> Self {
        Self {
            output: Box::new(WriteTestLayer::new()),
            input: Box::new(ReadTestLayer::new()),
        }
    }
}

trait TextOutput {
    fn write(&mut self, to_write: &str);
    #[cfg(test)]
    fn get_last_line(&self) -> String;
}

trait TextInput {
    fn read(&mut self) -> String;
    #[cfg(test)]
    fn set_input_text(&mut self, text: &str);
}

struct StdoutLayer<'a> {
    stdout: StdoutLock<'a>,
}

impl<'a> TextOutput for StdoutLayer<'a> {
    fn write(&mut self, to_write: &str) {
        print!("{to_write}");
        let _ = self.stdout.flush();
    }

    #[cfg(test)]
    fn get_last_line(&self) -> String {
        unreachable!();
    }
}

struct StdinLayer;
impl TextInput for StdinLayer {
    fn read(&mut self) -> String {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        let _ = handle.read_line(&mut buffer);
        buffer
    }

    #[cfg(test)]
    fn set_input_text(&mut self, _text: &str) {
        unreachable!();
    }
}

impl<'a> StdoutLayer<'a> {
    fn new() -> Box<Self> {
        Box::new(Self {
            stdout: std::io::stdout().lock(),
        })
    }
}
impl StdinLayer {
    fn new() -> Box<Self> {
        Box::new(Self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single() {
        let mut testio = IoLayer::test();
        testio.input.set_input_text("1,2");

        let result: (f32, f32) = testio.get_two_values("my prompt");

        assert_eq!(testio.output.get_last_line(), "my prompt? ".to_string());
        assert_eq!(1.0, result.0);
        assert_eq!(2.0, result.1);
    }

    #[test]
    #[ignore]
    fn test_re_enter() {
        let mut testio = IoLayer::test();
        testio.input.set_input_text("a");

        let result: f32 = testio.get_single_value("my prompt");

        assert_eq!(testio.output.get_last_line(), "my prompt? ".to_string());
        assert_eq!(1.0, result);
    }
}

#[cfg(test)]
struct ReadTestLayer {
    next_line: Option<String>,
}

#[cfg(test)]
impl ReadTestLayer {
    fn new() -> Self {
        Self { next_line: None }
    }
}

#[cfg(test)]
impl TextInput for ReadTestLayer {
    fn read(&mut self) -> String {
        if let Some(line) = self.next_line.take() {
            line
        } else {
            "".to_string()
        }
    }

    fn set_input_text(&mut self, text: &str) {
        self.next_line = Some(text.to_string());
    }
}

#[cfg(test)]
struct WriteTestLayer {
    last_item_written: String,
}

#[cfg(test)]
impl WriteTestLayer {
    fn new() -> Self {
        Self {
            last_item_written: String::new(),
        }
    }
}

#[cfg(test)]
impl TextOutput for WriteTestLayer {
    fn write(&mut self, to_write: &str) {
        self.last_item_written = to_write.into();
    }

    fn get_last_line(&self) -> String {
        self.last_item_written.clone()
    }
}
