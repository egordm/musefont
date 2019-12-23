pub type Point2F = euclid::default::Point2D<f32>;
pub type Size2F = euclid::default::Size2D<f32>;
pub type RectF = euclid::default::Rect<f32>;

macro_rules! impl_display {
    ($enum:ident, {$($variant:pat => $fmt_string:expr),+$(,)* }) => {

        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::$enum::*;
                match &self {
                    $(
                        $variant => write!(f, "{}", $fmt_string),
                    )+
                }
            }
        }
    };
}

#[derive(Debug)]
pub enum FontLoadingError {
	IO(std::io::Error),
	Font(font_kit::error::FontLoadingError),
	Glyph(font_kit::error::GlyphLoadingError),
	Json(json::Error),
}

impl std::error::Error for FontLoadingError {}

impl_display! { FontLoadingError, {
        IO(e) => format!("IO error: {}", e),
        Font(e) => format!("Font loading error: {}", e),
        Glyph(e) => format!("Glyph loading error: {}", e),
        Json(e) => format!("Json loading error: {}", e),
    }
}