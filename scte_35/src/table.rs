

/// This structure is used for logging/debugging purposes to print SCTE-35 descriptors in a tabular format.
pub struct Table<'a> {
    pub output: &'a mut Vec<String>,
}

impl<'a> Table<'a> {
    /// Creates a new Table instance to capture output in a Vec<String>.
    pub fn new(output: &'a mut Vec<String>) -> Self {
        Table { output }
    }

    /// Adds a row to the output (formatted as a table row).
    pub fn row(&mut self, indent: usize, label: &str, value: &str) {
        let indent_str = "  ".repeat(indent);
        self.output.push(format!("{}{}: {}", indent_str, label, value));
    }
}