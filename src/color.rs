use std::fmt;

use anstyle::Style;

pub fn note<D>(display: D) -> Styled<D> {
    Styled { display, style: clap_cargo::style::NOTE }
}

pub fn warn<D>(display: D) -> Styled<D> {
    Styled { display, style: clap_cargo::style::WARN }
}

#[derive(Debug)]
pub struct Styled<D> {
    display: D,
    style: Style,
}

impl<D: fmt::Display> fmt::Display for Styled<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.style.render())?;
        self.display.fmt(f)?;
        write!(f, "{}", self.style.render_reset())
    }
}
