use crate::x11api::{XDisplay, query_xrdb};

pub struct Xft {
    pub dpi: Option<i32>,
}

impl Xft {
    pub fn new() -> Self {
        let display = XDisplay::new()
            .expect("Failed to acquire X display!");
        let dpi = query_xrdb(*display, "Xft", "dpi")
            .and_then(|s| s.parse::<i32>().ok());
        Xft { dpi, }
    }
}


/// An arbitrary set of additional elements can be appended to the font name,
/// the complete list of possible properties is:
///
/// 	CPP constant	    Name		Type
/// 	----------------------------------------------
/// 	XFT_FAMILY	        family		String
/// 	XFT_STYLE	        style		String
/// 	XFT_SLANT	        slant		Int
/// 	XFT_WEIGHT	        weight		Int
/// 	XFT_SIZE	        size		Double
/// 	XFT_PIXEL_SIZE	    pixelsize		Double
/// 	XFT_ENCODING	    encoding		String
/// 	XFT_SPACING	        spacing		Int
/// 	XFT_FOUNDRY	        foundry		String
/// 	XFT_CORE	        core		Bool
/// 	XFT_ANTIALIAS	    antialias		Bool
/// 	XFT_XLFD	        xlfd		String
/// 	XFT_FILE	        file		String
/// 	XFT_INDEX	        index		Int
/// 	XFT_RASTERIZER	    rasterizer		String
/// 	XFT_OUTLINE	        outline		Bool
/// 	XFT_SCALABLE	    scalable		Bool
/// 	XFT_RGBA	        rgba		Int
/// 	(Defaults from resources)
/// 	XFT_SCALE	        scale		Double
/// 	XFT_RENDER	        render		Bool
/// 	XFT_MINSPACE	    minspace		Bool
/// 	(Specific to FreeType rasterizer)
/// 	XFT_CHAR_WIDTH	    charwidth		Int
/// 	XFT_CHAR_HEIGHT	    charheight		Int
/// 	XFT_MATRIX	        matrix    		XftMatrix
#[derive(Clone, Debug)]
pub struct FontName {
    pub family: String,
    pub style: String,
    pub slant: Slant,
    pub weight: Weight,
    pub size: f64,
    pub pixelsize: f64,
    pub encoding: String,
    pub spacing: Spacing,
    pub foundry: String,
    pub core: bool,
    pub antialias: bool,
    pub xlfd: String,
    pub file: String,
    pub index: i32,
    pub rasterizer: String,
    pub outline: bool,
    pub scalable: bool,
    pub rgba: Rgba,
    pub scale: f64,
    pub render: bool,
    pub minspace: bool,
// 	(Specific to FreeType rasterizer)
// 	XFT_CHAR_WIDTH	    charwidth		Int
// 	XFT_CHAR_HEIGHT	    charheight		Int
// 	XFT_MATRIX	        matrix    		XftMatrix
}

#[derive(Copy, Clone, Debug)]
pub enum Weight {
    Light, // 0
    Medium, // 100
    Demibold, // 180
    Bold, // 200
    Black, // 210
}

#[derive(Copy, Clone, Debug)]
pub enum Slant {
    Roman, // 0
    Italic, // 100
    Oblique, // 110
}

#[derive(Copy, Clone, Debug)]
pub enum Spacing {
    Proportional, // 0
    Mono, // 100
    Charcell, // 110
}

#[derive(Copy, Clone, Debug)]
pub enum Rgba {
    Rgb, // 1
    Bgr, // 2
    Vrgb, // 3
    Vbgr, // 4
}
