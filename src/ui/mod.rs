slint::include_modules!();

pub mod window;
pub mod handlers;
pub mod update_info;
pub mod online_status;
pub mod backgrounds;

pub type SharedConfig = std::rc::Rc<std::cell::RefCell<crate::config::Config>>;
