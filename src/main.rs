use log::info;
mod loader;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    let root = "testsite";
    info!("Loading site from {}", root);
    let l = loader::FileLoader::new(&root);
    let pt = l.to_site_dir()?;
    dbg!(pt);
    Ok(())
}
