use colorous::{
    // Sequential single-hue
    BLUES,
    // Sequential multi-hue (ColorBrewer)
    BLUE_GREEN,
    BLUE_PURPLE,
    // Diverging
    BROWN_GREEN,
    CIVIDIS,
    COOL,
    CUBEHELIX,
    GREENS,
    GREEN_BLUE,
    GREYS,
    INFERNO,
    MAGMA,
    ORANGES,
    ORANGE_RED,
    PINK_GREEN,
    PLASMA,
    PURPLES,
    PURPLE_BLUE,
    PURPLE_BLUE_GREEN,
    PURPLE_GREEN,
    PURPLE_ORANGE,
    PURPLE_RED,
    // Cyclical
    RAINBOW,
    REDS,
    RED_BLUE,
    RED_GREY,
    RED_PURPLE,
    RED_YELLOW_BLUE,
    RED_YELLOW_GREEN,
    SINEBOW,
    SPECTRAL,
    // Sequential multi-hue (perceptual)
    TURBO,
    VIRIDIS,
    WARM,
    YELLOW_GREEN,
    YELLOW_GREEN_BLUE,
    YELLOW_ORANGE_BROWN,
    YELLOW_ORANGE_RED,
};
use std::sync::Arc;

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Convert an RGB triplet to a 7-byte hex color string (`#rrggbb`).
/// Avoids `format!` overhead in hot loops (heatmaps, 2D histograms).
#[inline]
pub(crate) fn rgb_hex(r: u8, g: u8, b: u8) -> String {
    let bytes = [
        b'#',
        HEX_DIGITS[(r >> 4) as usize],
        HEX_DIGITS[(r & 0xf) as usize],
        HEX_DIGITS[(g >> 4) as usize],
        HEX_DIGITS[(g & 0xf) as usize],
        HEX_DIGITS[(b >> 4) as usize],
        HEX_DIGITS[(b & 0xf) as usize],
    ];
    // SAFETY: all bytes are ASCII
    unsafe { String::from_utf8_unchecked(bytes.to_vec()) }
}

fn cmap_str(gradient: colorous::Gradient, value: f64) -> String {
    let rgb = gradient.eval_continuous(value.clamp(0.0, 1.0));
    rgb_hex(rgb.r, rgb.g, rgb.b)
}

fn cmap_rgb(gradient: colorous::Gradient, value: f64) -> (u8, u8, u8) {
    let rgb = gradient.eval_continuous(value.clamp(0.0, 1.0));
    (rgb.r, rgb.g, rgb.b)
}

/// Color map used to encode numeric cell values as colors.
///
/// Values are normalized to `[0.0, 1.0]` relative to the data min/max before
/// the map is applied. Used by [`Heatmap`](crate::plot::Heatmap),
/// [`Histogram2D`](crate::plot::Histogram2D), and
/// [`CalendarPlot`](crate::plot::CalendarPlot).
///
/// # Choosing a color map
///
/// | Category | Variants | Use when |
/// |----------|----------|----------|
/// | Sequential (perceptual) | `Viridis`, `Inferno`, `Magma`, `Plasma`, `Cividis`, `Turbo`, `Warm`, `Cool`, `Cubehelix` | General-purpose continuous data; colorblind-safe options |
/// | Sequential (ColorBrewer) | `BlueGreen`, `BluePurple`, `GreenBlue`, `OrangeRed`, `PurpleBlue`, `PurpleBlueGreen`, `PurpleRed`, `RedPurple`, `YellowGreen`, `YellowGreenBlue`, `YellowOrangeBrown`, `YellowOrangeRed` | Themed sequential scales from [ColorBrewer](https://colorbrewer2.org/) |
/// | Sequential (single-hue) | `Blues`, `Greens`, `Grayscale`, `Oranges`, `Purples`, `Reds` | Monochromatic; print-friendly |
/// | Diverging | `BrownGreen`, `PinkGreen`, `PurpleGreen`, `PurpleOrange`, `RedBlue`, `RedGrey`, `RedYellowBlue`, `RedYellowGreen`, `Spectral` | Data with a meaningful midpoint (e.g. fold-change, correlation) |
/// | Cyclical | `Rainbow`, `Sinebow` | Periodic data (phase, angle, time-of-day) |
/// | Custom | `Custom` | Full control over color encoding |
#[derive(Clone)]
pub enum ColorMap {
    // ── Sequential multi-hue (perceptual) ──────────────────────────────────
    /// Improved rainbow; perceptually uniform; colorblind-safe.
    Turbo,
    /// Blue → green → yellow; perceptually uniform; default for most plots.
    Viridis,
    /// Black → purple → yellow; high-contrast; works in greyscale.
    Inferno,
    /// Black → purple → orange; similar to Inferno.
    Magma,
    /// Blue → purple → yellow; bright and perceptually uniform.
    Plasma,
    /// Blue → grey → yellow; optimized for color-vision deficiency.
    Cividis,
    /// Warm perceptual rainbow (180° rotation of Cool).
    Warm,
    /// Cool perceptual rainbow.
    Cool,
    /// Green's default Cubehelix spiral.
    Cubehelix,

    // ── Sequential multi-hue (ColorBrewer) ────────────────────────────────
    /// White → blue-green.
    BlueGreen,
    /// White → blue-purple.
    BluePurple,
    /// White → green-blue.
    GreenBlue,
    /// White → orange-red.
    OrangeRed,
    /// White → purple-blue-green.
    PurpleBlueGreen,
    /// White → purple-blue.
    PurpleBlue,
    /// White → purple-red.
    PurpleRed,
    /// White → red-purple.
    RedPurple,
    /// White → yellow-green-blue.
    YellowGreenBlue,
    /// White → yellow-green.
    YellowGreen,
    /// White → yellow-orange-brown.
    YellowOrangeBrown,
    /// White → yellow-orange-red.
    YellowOrangeRed,

    // ── Sequential single-hue ─────────────────────────────────────────────
    /// White → blue.
    Blues,
    /// White → green.
    Greens,
    /// White → black; print-friendly.
    Grayscale,
    /// White → orange.
    Oranges,
    /// White → purple.
    Purples,
    /// White → red.
    Reds,

    // ── Diverging ─────────────────────────────────────────────────────────
    /// Brown ← 0 → green.
    BrownGreen,
    /// Pink ← 0 → green.
    PinkGreen,
    /// Purple ← 0 → green.
    PurpleGreen,
    /// Purple ← 0 → orange.
    PurpleOrange,
    /// Red ← 0 → blue.
    RedBlue,
    /// Red ← 0 → grey.
    RedGrey,
    /// Red ← 0 → yellow → blue.
    RedYellowBlue,
    /// Red ← 0 → yellow → green.
    RedYellowGreen,
    /// Red → orange → yellow → green → blue → purple.
    Spectral,

    // ── Cyclical ──────────────────────────────────────────────────────────
    /// Less-angry rainbow; suitable for cyclical data.
    Rainbow,
    /// Smooth sinusoidal rainbow.
    Sinebow,

    // ── Custom ────────────────────────────────────────────────────────────
    /// User-defined mapping from a normalized `[0.0, 1.0]` value to a CSS
    /// color string. Wrap the function in `Arc` for cloneability.
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use kuva::plot::ColorMap;
    ///
    /// // Custom blue-to-red diverging scale
    /// let cmap = ColorMap::Custom(Arc::new(|t: f64| {
    ///     let r = (t * 255.0) as u8;
    ///     let b = ((1.0 - t) * 255.0) as u8;
    ///     format!("rgb({r},0,{b})")
    /// }));
    /// ```
    Custom(Arc<dyn Fn(f64) -> String + Send + Sync>),
}

impl std::fmt::Debug for ColorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ColorMap::Turbo => "Turbo",
            ColorMap::Viridis => "Viridis",
            ColorMap::Inferno => "Inferno",
            ColorMap::Magma => "Magma",
            ColorMap::Plasma => "Plasma",
            ColorMap::Cividis => "Cividis",
            ColorMap::Warm => "Warm",
            ColorMap::Cool => "Cool",
            ColorMap::Cubehelix => "Cubehelix",
            ColorMap::BlueGreen => "BlueGreen",
            ColorMap::BluePurple => "BluePurple",
            ColorMap::GreenBlue => "GreenBlue",
            ColorMap::OrangeRed => "OrangeRed",
            ColorMap::PurpleBlueGreen => "PurpleBlueGreen",
            ColorMap::PurpleBlue => "PurpleBlue",
            ColorMap::PurpleRed => "PurpleRed",
            ColorMap::RedPurple => "RedPurple",
            ColorMap::YellowGreenBlue => "YellowGreenBlue",
            ColorMap::YellowGreen => "YellowGreen",
            ColorMap::YellowOrangeBrown => "YellowOrangeBrown",
            ColorMap::YellowOrangeRed => "YellowOrangeRed",
            ColorMap::Blues => "Blues",
            ColorMap::Greens => "Greens",
            ColorMap::Grayscale => "Grayscale",
            ColorMap::Oranges => "Oranges",
            ColorMap::Purples => "Purples",
            ColorMap::Reds => "Reds",
            ColorMap::BrownGreen => "BrownGreen",
            ColorMap::PinkGreen => "PinkGreen",
            ColorMap::PurpleGreen => "PurpleGreen",
            ColorMap::PurpleOrange => "PurpleOrange",
            ColorMap::RedBlue => "RedBlue",
            ColorMap::RedGrey => "RedGrey",
            ColorMap::RedYellowBlue => "RedYellowBlue",
            ColorMap::RedYellowGreen => "RedYellowGreen",
            ColorMap::Spectral => "Spectral",
            ColorMap::Rainbow => "Rainbow",
            ColorMap::Sinebow => "Sinebow",
            ColorMap::Custom(_) => return write!(f, "ColorMap::Custom(<fn>)"),
        };
        write!(f, "ColorMap::{name}")
    }
}

impl ColorMap {
    /// Map a normalized value in `[0.0, 1.0]` to a CSS color string.
    pub fn map(&self, value: f64) -> String {
        match self {
            ColorMap::Turbo => cmap_str(TURBO, value),
            ColorMap::Viridis => cmap_str(VIRIDIS, value),
            ColorMap::Inferno => cmap_str(INFERNO, value),
            ColorMap::Magma => cmap_str(MAGMA, value),
            ColorMap::Plasma => cmap_str(PLASMA, value),
            ColorMap::Cividis => cmap_str(CIVIDIS, value),
            ColorMap::Warm => cmap_str(WARM, value),
            ColorMap::Cool => cmap_str(COOL, value),
            ColorMap::Cubehelix => cmap_str(CUBEHELIX, value),
            ColorMap::BlueGreen => cmap_str(BLUE_GREEN, value),
            ColorMap::BluePurple => cmap_str(BLUE_PURPLE, value),
            ColorMap::GreenBlue => cmap_str(GREEN_BLUE, value),
            ColorMap::OrangeRed => cmap_str(ORANGE_RED, value),
            ColorMap::PurpleBlueGreen => cmap_str(PURPLE_BLUE_GREEN, value),
            ColorMap::PurpleBlue => cmap_str(PURPLE_BLUE, value),
            ColorMap::PurpleRed => cmap_str(PURPLE_RED, value),
            ColorMap::RedPurple => cmap_str(RED_PURPLE, value),
            ColorMap::YellowGreenBlue => cmap_str(YELLOW_GREEN_BLUE, value),
            ColorMap::YellowGreen => cmap_str(YELLOW_GREEN, value),
            ColorMap::YellowOrangeBrown => cmap_str(YELLOW_ORANGE_BROWN, value),
            ColorMap::YellowOrangeRed => cmap_str(YELLOW_ORANGE_RED, value),
            ColorMap::Blues => cmap_str(BLUES, value),
            ColorMap::Greens => cmap_str(GREENS, value),
            ColorMap::Grayscale => cmap_str(GREYS, value),
            ColorMap::Oranges => cmap_str(ORANGES, value),
            ColorMap::Purples => cmap_str(PURPLES, value),
            ColorMap::Reds => cmap_str(REDS, value),
            ColorMap::BrownGreen => cmap_str(BROWN_GREEN, value),
            ColorMap::PinkGreen => cmap_str(PINK_GREEN, value),
            ColorMap::PurpleGreen => cmap_str(PURPLE_GREEN, value),
            ColorMap::PurpleOrange => cmap_str(PURPLE_ORANGE, value),
            ColorMap::RedBlue => cmap_str(RED_BLUE, value),
            ColorMap::RedGrey => cmap_str(RED_GREY, value),
            ColorMap::RedYellowBlue => cmap_str(RED_YELLOW_BLUE, value),
            ColorMap::RedYellowGreen => cmap_str(RED_YELLOW_GREEN, value),
            ColorMap::Spectral => cmap_str(SPECTRAL, value),
            ColorMap::Rainbow => cmap_str(RAINBOW, value),
            ColorMap::Sinebow => cmap_str(SINEBOW, value),
            ColorMap::Custom(f) => f(value),
        }
    }

    /// Map a normalized value to `(r, g, b)` bytes, avoiding string allocation.
    /// Returns `None` for `Custom` colormaps (which must go through `map()`).
    pub fn map_rgb(&self, value: f64) -> Option<(u8, u8, u8)> {
        Some(match self {
            ColorMap::Turbo => cmap_rgb(TURBO, value),
            ColorMap::Viridis => cmap_rgb(VIRIDIS, value),
            ColorMap::Inferno => cmap_rgb(INFERNO, value),
            ColorMap::Magma => cmap_rgb(MAGMA, value),
            ColorMap::Plasma => cmap_rgb(PLASMA, value),
            ColorMap::Cividis => cmap_rgb(CIVIDIS, value),
            ColorMap::Warm => cmap_rgb(WARM, value),
            ColorMap::Cool => cmap_rgb(COOL, value),
            ColorMap::Cubehelix => cmap_rgb(CUBEHELIX, value),
            ColorMap::BlueGreen => cmap_rgb(BLUE_GREEN, value),
            ColorMap::BluePurple => cmap_rgb(BLUE_PURPLE, value),
            ColorMap::GreenBlue => cmap_rgb(GREEN_BLUE, value),
            ColorMap::OrangeRed => cmap_rgb(ORANGE_RED, value),
            ColorMap::PurpleBlueGreen => cmap_rgb(PURPLE_BLUE_GREEN, value),
            ColorMap::PurpleBlue => cmap_rgb(PURPLE_BLUE, value),
            ColorMap::PurpleRed => cmap_rgb(PURPLE_RED, value),
            ColorMap::RedPurple => cmap_rgb(RED_PURPLE, value),
            ColorMap::YellowGreenBlue => cmap_rgb(YELLOW_GREEN_BLUE, value),
            ColorMap::YellowGreen => cmap_rgb(YELLOW_GREEN, value),
            ColorMap::YellowOrangeBrown => cmap_rgb(YELLOW_ORANGE_BROWN, value),
            ColorMap::YellowOrangeRed => cmap_rgb(YELLOW_ORANGE_RED, value),
            ColorMap::Blues => cmap_rgb(BLUES, value),
            ColorMap::Greens => cmap_rgb(GREENS, value),
            ColorMap::Grayscale => cmap_rgb(GREYS, value),
            ColorMap::Oranges => cmap_rgb(ORANGES, value),
            ColorMap::Purples => cmap_rgb(PURPLES, value),
            ColorMap::Reds => cmap_rgb(REDS, value),
            ColorMap::BrownGreen => cmap_rgb(BROWN_GREEN, value),
            ColorMap::PinkGreen => cmap_rgb(PINK_GREEN, value),
            ColorMap::PurpleGreen => cmap_rgb(PURPLE_GREEN, value),
            ColorMap::PurpleOrange => cmap_rgb(PURPLE_ORANGE, value),
            ColorMap::RedBlue => cmap_rgb(RED_BLUE, value),
            ColorMap::RedGrey => cmap_rgb(RED_GREY, value),
            ColorMap::RedYellowBlue => cmap_rgb(RED_YELLOW_BLUE, value),
            ColorMap::RedYellowGreen => cmap_rgb(RED_YELLOW_GREEN, value),
            ColorMap::Spectral => cmap_rgb(SPECTRAL, value),
            ColorMap::Rainbow => cmap_rgb(RAINBOW, value),
            ColorMap::Sinebow => cmap_rgb(SINEBOW, value),
            ColorMap::Custom(_) => return None,
        })
    }
}
