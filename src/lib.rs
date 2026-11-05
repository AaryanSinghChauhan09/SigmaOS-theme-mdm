#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreeterTheme {
    pub name: String,
    pub index_html: String,
}

pub struct MdmThemeEngine;

impl MdmThemeEngine {
    pub fn load_theme(name: &str) -> GreeterTheme {
        GreeterTheme {
            name: name.to_string(),
            index_html: format!("/usr/share/mdm/themes/{}/index.html", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mdm_theme_engine_aaa() {
        // Arrange & Act
        let theme = MdmThemeEngine::load_theme("sigmaos-dark");
        // Assert
        assert_eq!(theme.name, "sigmaos-dark");
        assert!(theme.index_html.contains("sigmaos-dark"));
    }
}
