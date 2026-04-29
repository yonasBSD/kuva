/// Alignment for text within a [`TextPlot`].
#[derive(Debug, Clone, Copy, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// A plot cell that renders formatted, word-wrapped text.
///
/// Supports a title, body text with basic markup, optional background, and a border.
///
/// **Markup syntax** (line-level only):
/// - `# Heading` — large bold heading
/// - `## Subheading` — medium bold heading
/// - `**bold line**` — bold paragraph
/// - `---` — horizontal rule
/// - Blank line — paragraph spacing
#[derive(Debug, Clone)]
pub struct TextPlot {
    pub body: String,
    pub title: Option<String>,
    pub font_size: Option<u32>,
    pub padding: f64,
    pub background: Option<String>,
    pub border_color: Option<String>,
    pub border_width: f64,
    pub text_align: TextAlign,
    pub text_color: Option<String>,
}

impl Default for TextPlot {
    fn default() -> Self { Self::new() }
}

impl TextPlot {
    pub fn new() -> Self {
        Self {
            body: String::new(),
            title: None,
            font_size: None,
            padding: 16.0,
            background: None,
            border_color: None,
            border_width: 0.0,
            text_align: TextAlign::Left,
            text_color: None,
        }
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_background(mut self, color: impl Into<String>) -> Self {
        self.background = Some(color.into());
        self
    }

    pub fn with_border(mut self, color: impl Into<String>, width: f64) -> Self {
        self.border_color = Some(color.into());
        self.border_width = width;
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.text_align = align;
        self
    }

    pub fn with_text_color(mut self, color: impl Into<String>) -> Self {
        self.text_color = Some(color.into());
        self
    }
}
