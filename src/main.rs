use command::command::new_command;
use tracing::Level;


fn main() {
    tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();
    new_command(); 
}
