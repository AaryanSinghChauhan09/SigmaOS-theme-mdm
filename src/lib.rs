//! SigmaOS-theme-mdm — Display manager (MDM/SDDM/GDM) theme for SigmaOS.

/// Greeter theme
#[derive(Debug, Clone)]
pub struct GreeterTheme {
    pub name: String,
    pub description: String,
    pub background_image: Option<String>,
    pub background_color: String,
    pub logo_image: Option<String>,
    pub title_color: String,
    pub text_color: String,
    pub button_color: String,
    pub button_hover_color: String,
    pub font_family: String,
    pub font_size: u32,
    pub animation: GreeterAnimation,
    pub show_clock: bool,
    pub show_user_list: bool,
    pub index_html: String,
}

/// Greeter animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreeterAnimation {
    /// No animation
    None,
    /// Fade in
    FadeIn,
    /// Slide
    Slide,
    /// Zoom
    Zoom,
}

impl GreeterTheme {
    /// Create new theme
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            background_image: None,
            background_color: "#1a1a2e".into(),
            logo_image: None,
            title_color: "#ffffff".into(),
            text_color: "#cccccc".into(),
            button_color: "#7c3aed".into(),
            button_hover_color: "#8b5cf6".into(),
            font_family: "Sans".into(),
            font_size: 14,
            animation: GreeterAnimation::FadeIn,
            show_clock: true,
            show_user_list: true,
            index_html: format!("/usr/share/mdm/themes/{}/index.html", name),
        }
    }

    /// Set background
    pub fn set_background(&mut self, image: Option<&str>, color: &str) {
        self.background_image = image.map(|s| s.into());
        self.background_color = color.into();
    }

    /// Set logo
    pub fn set_logo(&mut self, image: Option<&str>) {
        self.logo_image = image.map(|s| s.into());
    }

    /// Set colors
    pub fn set_colors(&mut self, title: &str, text: &str, button: &str, hover: &str) {
        self.title_color = title.into();
        self.text_color = text.into();
        self.button_color = button.into();
        self.button_hover_color = hover.into();
    }

    /// Set font
    pub fn set_font(&mut self, family: &str, size: u32) {
        self.font_family = family.into();
        self.font_size = size;
    }

    /// Set animation
    pub fn set_animation(&mut self, anim: GreeterAnimation) {
        self.animation = anim;
    }

    /// Toggle clock
    pub fn toggle_clock(&mut self) { self.show_clock = !self.show_clock; }

    /// Toggle user list
    pub fn toggle_user_list(&mut self) { self.show_user_list = !self.show_user_list; }

    /// Serialize to config
    pub fn to_config(&self) -> String {
        format!(
            "name={}\nbg={}\ntitle={}\ntext={}\nbutton={}\nhover={}\nfont={}\nsize={}\nanimation={:?}\nclock={}\nusers={}\n",
            self.name, self.background_color, self.title_color, self.text_color,
            self.button_color, self.button_hover_color, self.font_family, self.font_size,
            self.animation, self.show_clock, self.show_user_list
        )
    }
}

/// MDM theme engine
#[derive(Debug, Default)]
pub struct MdmThemeEngine {
    themes: Vec<GreeterTheme>,
    active: Option<String>,
}

impl MdmThemeEngine {
    /// Create new engine
    pub fn new() -> Self { Self::default() }

    /// Load theme (legacy compat)
    pub fn load_theme(name: &str) -> GreeterTheme {
        GreeterTheme::new(name)
    }

    /// Register theme
    pub fn register(&mut self, theme: GreeterTheme) -> bool {
        if self.themes.iter().any(|t| t.name == theme.name) { return false; }
        self.themes.push(theme);
        true
    }

    /// Activate theme
    pub fn activate(&mut self, name: &str) -> bool {
        if self.themes.iter().any(|t| t.name == name) {
            self.active = Some(name.into());
            return true;
        }
        false
    }

    /// Get active theme
    pub fn active(&self) -> Option<&GreeterTheme> {
        self.active.as_ref().and_then(|n| self.themes.iter().find(|t| &t.name == n))
    }

    /// List themes
    pub fn list(&self) -> &[GreeterTheme] { &self.themes }

    /// Default themes
    pub fn with_defaults() -> Self {
        let mut engine = Self::new();
        engine.register(GreeterTheme::new("sigmaos-dark"));
        engine.register(GreeterTheme::new("sigmaos-light"));
        engine.activate("sigmaos-dark");
        engine
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeter_animation() {
        let anims = vec![GreeterAnimation::None, GreeterAnimation::FadeIn, GreeterAnimation::Slide, GreeterAnimation::Zoom];
        assert_eq!(anims.len(), 4);
    }

    #[test]
    fn test_theme_creation() {
        let theme = GreeterTheme::new("test");
        assert_eq!(theme.name, "test");
        assert_eq!(theme.animation, GreeterAnimation::FadeIn);
        assert!(theme.show_clock);
        assert!(theme.show_user_list);
        assert!(theme.index_html.contains("test"));
    }

    #[test]
    fn test_set_background() {
        let mut theme = GreeterTheme::new("t");
        theme.set_background(Some("/bg.png"), "#000");
        assert_eq!(theme.background_image, Some("/bg.png".to_string()));
        assert_eq!(theme.background_color, "#000");
    }

    #[test]
    fn test_set_colors() {
        let mut theme = GreeterTheme::new("t");
        theme.set_colors("#fff", "#ccc", "#f00", "#f80");
        assert_eq!(theme.title_color, "#fff");
        assert_eq!(theme.button_color, "#f00");
    }

    #[test]
    fn test_toggles() {
        let mut theme = GreeterTheme::new("t");
        theme.toggle_clock();
        assert!(!theme.show_clock);
        theme.toggle_user_list();
        assert!(!theme.show_user_list);
    }

    #[test]
    fn test_to_config() {
        let theme = GreeterTheme::new("cfg");
        let cfg = theme.to_config();
        assert!(cfg.contains("name=cfg"));
        assert!(cfg.contains("animation=FadeIn"));
    }

    #[test]
    fn test_engine() {
        let mut engine = MdmThemeEngine::new();
        assert!(engine.register(GreeterTheme::new("a")));
        assert!(!engine.register(GreeterTheme::new("a"))); // dup
        assert!(engine.activate("a"));
        assert_eq!(engine.active().unwrap().name, "a");
        assert_eq!(MdmThemeEngine::with_defaults().list().len(), 2);
    }

    #[test]
    fn test_load_theme_legacy() {
        let theme = MdmThemeEngine::load_theme("sigmaos-dark");
        assert_eq!(theme.name, "sigmaos-dark");
        assert!(theme.index_html.contains("sigmaos-dark"));
    }
}
