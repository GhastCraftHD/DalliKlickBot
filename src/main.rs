use tracing::info;

mod logging;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging()?;
    info!("Starting DalliKlickBot...");
    
    Ok(())    
}
