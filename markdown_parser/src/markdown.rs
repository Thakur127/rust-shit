use regex::Regex;

pub struct Markdown {
    pub content: String,
}

impl Markdown {
    pub fn to_html(&self) -> String {
        let mut output = String::new();

        for line in self.content.lines() {
            if line.starts_with('#') {
                output += &self.parse_header(line);
            } else if line.is_empty() {
                output += "\n\r";
            } else if line.starts_with('!') {
                output += &self.parse_image(line);
            } else if line.starts_with('-') || line.starts_with('*') {
                output += &self.parse_list(line);
            } else {
                output += &self.parse_paragraph(line);
            }
        }

        output
    }

    fn parse_header(&self, input: &str) -> String {
        let mut level = 0;

        for char in input.chars() {
            if char == '#' {
                level += 1;
            } else if char == ' ' {
                break;
            }
        }

        if level > 0 && level < 7 {
            format!(
                "<h{level}>{}</h{level}>\n\r",
                input.trim_matches(|c: char| c == '#' || c.is_whitespace())
            )
        } else {
            self.parse_paragraph(input)
        }
    }

    fn parse_paragraph(&self, input: &str) -> String {
        let mut parsed = input.to_string();
        parsed = self.parse_bold(&parsed);
        parsed = self.parse_italic(&parsed);
        parsed = self.parse_link(&parsed);

        format!("<p>{}</p>\n\r", parsed)
    }

    fn parse_bold(&self, input: &str) -> String {
        let b_re = Regex::new(r"\*\*(.*?)\*\*").unwrap();
        let strong_re = Regex::new(r"__([^_]+)__").unwrap();

        let input = b_re.replace_all(input, |caps: &regex::Captures| {
            format!("<b>{}</b>", &caps[1])
        });

        let input = strong_re.replace_all(&input, |caps: &regex::Captures| {
            format!("<strong>{}</strong>", &caps[1])
        });

        input.into()
    }

    fn parse_italic(&self, input: &str) -> String {
        let em_re = Regex::new(r"\_(.*?)\_").unwrap();
        let i_re = Regex::new(r"\*(.*?)\*").unwrap();

        let input = i_re.replace_all(input, |caps: &regex::Captures| {
            format!("<i>{}</i>", &caps[1])
        });

        let input = em_re.replace_all(&input, |caps: &regex::Captures| {
            format!("<em>{}</em>", &caps[1])
        });

        input.into()
    }

    fn parse_link(&self, input: &str) -> String {
        let re = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        re.replace_all(input.trim(), |caps: &regex::Captures| {
            format!("<a href=\"{}\">{}</a>", &caps[2], &caps[1])
        })
        .into()
    }

    fn parse_image(&self, input: &str) -> String {
        let re = Regex::new(r"!\[([^\]]+)\]\(([^)]+)\)").unwrap();
        re.replace_all(input.trim(), |caps: &regex::Captures| {
            format!("<img src=\"{}\" alt=\"{}\" />\n\r", &caps[2], &caps[1])
        })
        .into()
    }

    fn parse_list(&self, input: &str) -> String {
        let re = Regex::new(r"^\s*[-\*]\s*(.*)").unwrap();
        if let Some(caps) = re.captures(input) {
            return format!("<li>{}</li>\n", &caps[1]);
        }
        input.to_string()
    }
}
