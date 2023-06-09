pub trait Node {
    fn represent(&self) -> String;
}

use core::fmt::Debug;
impl Debug for dyn Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.represent())
    }
}
