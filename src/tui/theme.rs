use ratatui::style::Color;

/// FlexPrice terminal theme — dark, modern, premium
pub struct Theme;

impl Theme {
    // ─── Brand Colors ─────────────────────────────────
    pub const PRIMARY: Color = Color::Rgb(99, 102, 241);    // Electric indigo
    pub const ACCENT: Color = Color::Rgb(16, 185, 129);     // Emerald
    pub const WARNING: Color = Color::Rgb(245, 158, 11);    // Amber
    pub const ERROR: Color = Color::Rgb(244, 63, 94);       // Rose
    pub const INFO: Color = Color::Rgb(56, 189, 248);       // Sky blue

    // ─── Surfaces ─────────────────────────────────────
    pub const BG: Color = Color::Rgb(15, 23, 42);           // Deep slate
    pub const SURFACE: Color = Color::Rgb(30, 41, 59);      // Slate
    pub const SURFACE_HOVER: Color = Color::Rgb(51, 65, 85);// Lighter slate
    pub const BORDER: Color = Color::Rgb(71, 85, 105);      // Slate border

    // ─── Text ─────────────────────────────────────────
    pub const TEXT: Color = Color::Rgb(226, 232, 240);       // Cool gray
    pub const TEXT_DIM: Color = Color::Rgb(148, 163, 184);   // Dim gray
    pub const TEXT_MUTED: Color = Color::Rgb(100, 116, 139); // Muted
}
