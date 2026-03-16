extern crate winres;

fn main() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("src/assets/computer2.ico"); 

        res.compile()?;
    }
    Ok(())
}