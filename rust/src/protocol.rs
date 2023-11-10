use regex::Regex;

pub fn parse_line(line: String) -> Option<u16> {
    let re = Regex::new(r"^L(?P<light>\d+)\r\n").unwrap();

    re.captures(line.as_str())
        .map(|caps| caps["light"].parse::<u16>().ok())
        .flatten()
        .map(|brightness| (brightness).saturating_sub(25) / 3)
}
