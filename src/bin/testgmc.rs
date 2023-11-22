#[macro_use] extern crate tracing;
extern crate tokio;



use gqgmclib::GMC;
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mut gmc = GMC::new("COM3", 57600).unwrap();
    let version = &gmc.get_version().await.unwrap();
    let cpm = &gmc.get_cpm().await.unwrap();
    info!("Device is {version}, CPM is {cpm}");
}