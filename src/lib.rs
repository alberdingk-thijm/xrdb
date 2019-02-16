// x11api code is taken from github.com/jD91mZM2/xidlehook
mod x11api;
mod colors;
mod xft;

pub use crate::colors::Colors;
pub use crate::x11api::{XDisplay, get_xrm_resource, query_xrdb};
pub use crate::xft::{Xft, FontName};

#[cfg(test)]
mod tests {
    #[test]
    fn xterm_colors() {
        use crate::colors::Colors;
        let xterm = Colors::new("xterm");
        // for debugging
        println!("{:?}", xterm);
        assert!(xterm.is_some());
    }
}
