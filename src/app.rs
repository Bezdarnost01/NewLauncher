use crate::{ui};

use slint::ComponentHandle;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new()?;

    let app = ui::window::create(&runtime)?;

    app.run()?;

    Ok(())
}
