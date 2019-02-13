use crate::x11api::{XDisplay, get_xrm_resource};
use x11::xlib::{XrmDatabase, XResourceManagerString, XrmGetStringDatabase, XrmDestroyDatabase};
use scopeguard::defer;

/// A struct for representing the colours found in the X Resource Manager's database.
#[derive(Clone, Debug, Default)]
pub struct Colors {
    /// Foreground colour: matches the "foreground" key.
    pub fg: Option<String>,
    /// Background colour: matches the "background" key.
    pub bg: Option<String>,
    /// Cursor colour: matches the "cursorColor" key.
    pub cursor: Option<String>,
    /// Colours 0 to 15: match the "color{}" keys, where {} is a number from 0 to 15 inclusive.
    pub colors: [Option<String>; 16],
}

impl Colors {
    /// Return an Colors struct if one can be obtained from the X Resource Manager.
    /// The class specifies the part of the database tree to look under.
    /// Giving the class "xterm" will return a struct filled with the values of the keys
    ///
    /// * xterm.foreground
    /// * xterm.background
    /// * xterm.cursorColor
    /// * xterm.color1 through xterm.color15
    ///
    /// If any of those keys are missing, the corresponding field will be set to None.
    pub fn new<'a>(class: &'a str) -> Option<Self> {
        let display = XDisplay::new()
            .expect("Failed to acquire X display!");
        unsafe {
            let rms = XResourceManagerString(*display);
            if !rms.is_null() {
                let db = XrmGetStringDatabase(rms);
                if !db.is_null() {
                    defer!({
                        XrmDestroyDatabase(db);
                    });
                    return Some(Colors::from_database(db, class));
                }
            }
        }
        None
    }

    unsafe fn from_database<'a>(db: XrmDatabase, class: &'a str) -> Self {
        let mut xcolors = Colors::default();
        let fg = get_xrm_resource(db, class, "foreground").map(|s| String::from(s));
        let bg = get_xrm_resource(db, class, "background").map(|s| String::from(s));
        let cursor = get_xrm_resource(db, class, "cursorColor").map(|s| String::from(s));
        let color_names = (0..16).map(|i| format!("color{}", i));
        let colors = color_names.map(|s| get_xrm_resource(db, class, &s).map(|s| String::from(s))).collect::<Vec<_>>();
        xcolors.fg = fg;
        xcolors.bg = bg;
        xcolors.cursor = cursor;
        xcolors.colors = [
            colors[0].clone(), colors[1].clone(), colors[2].clone(), colors[3].clone(),
            colors[4].clone(), colors[5].clone(), colors[6].clone(), colors[7].clone(),
            colors[8].clone(), colors[9].clone(), colors[10].clone(), colors[11].clone(),
            colors[12].clone(), colors[13].clone(), colors[14].clone(), colors[15].clone(),
        ];
        xcolors
    }
}
