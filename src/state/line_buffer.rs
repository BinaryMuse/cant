use std::cmp::min;

pub struct LogBuffer {
    pub lines: Vec<String>,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn get_lines(&self, start: u16, count: u16) -> Vec<String> {
        if self.lines.is_empty() {
            return Vec::new();
        }

        let start = start as usize;
        let end = min(self.lines.len(), start + count as usize);
        self.lines[start..end].to_vec()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lines_normal() {
        let mut buffer = LogBuffer::new();
        buffer.add_line("line 1".to_string());
        buffer.add_line("line 2".to_string());
        buffer.add_line("line 3".to_string());
        buffer.add_line("line 4".to_string());
        buffer.add_line("line 5".to_string());
        buffer.add_line("line 6".to_string());
        buffer.add_line("line 7".to_string());
        buffer.add_line("line 8".to_string());
        buffer.add_line("line 9".to_string());
        buffer.add_line("line 10".to_string());

        let lines = buffer.get_lines(1, 3);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 2".to_string());
        assert_eq!(lines[1], "line 3".to_string());
        assert_eq!(lines[2], "line 4".to_string());
    }

    #[test]
    fn test_get_lines_overflow() {
        let mut buffer = LogBuffer::new();
        buffer.add_line("line 1".to_string());
        buffer.add_line("line 2".to_string());
        buffer.add_line("line 3".to_string());
        buffer.add_line("line 4".to_string());
        buffer.add_line("line 5".to_string());
        buffer.add_line("line 6".to_string());
        buffer.add_line("line 7".to_string());
        buffer.add_line("line 8".to_string());
        buffer.add_line("line 9".to_string());
        buffer.add_line("line 10".to_string());

        let lines = buffer.get_lines(7, 4);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line 8".to_string());
        assert_eq!(lines[1], "line 9".to_string());
        assert_eq!(lines[2], "line 10".to_string());
    }
}
