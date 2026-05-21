mod ui;
mod app;
mod integrations;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
