mod topbar;

pub use topbar::Topbar;

pub trait Component {
    fn render(&self) -> String;
}
