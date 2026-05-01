use crate::plot::legend::LegendEntry;

pub struct LegendPlot {
    pub entries: Vec<LegendEntry>,
    pub cols: Option<usize>,
    pub title: Option<String>,
    pub show_box: bool,
}

impl LegendPlot {
    pub fn new() -> Self {
        Self { entries: Vec::new(), cols: None, title: None, show_box: true }
    }

    pub fn from_entries(entries: Vec<LegendEntry>) -> Self {
        Self { entries, cols: None, title: None, show_box: true }
    }

    pub fn with_entry(mut self, entry: LegendEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn with_cols(mut self, n: usize) -> Self {
        self.cols = Some(n);
        self
    }

    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn without_box(mut self) -> Self {
        self.show_box = false;
        self
    }
}

impl Default for LegendPlot {
    fn default() -> Self { Self::new() }
}
