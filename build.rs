extern crate winres;

fn main() -> std::io::Result<()> {
    if cfg!(target_os = "windows") {
        // 1. Crea una nuova configurazione di risorse
        let mut res = winres::WindowsResource::new();

        // 2. Specifica il percorso dell'icona (winres accetta l'icona direttamente)
        // NOTA: winres può usare l'icona direttamente, senza bisogno del file app.rc.
        // Se non hai altre risorse oltre l'icona, puoi fare a meno di app.rc!
        res.set_icon("src/assets/computer2.ico"); 

        // 3. Esegui la compilazione
        res.compile()?;
    }
    Ok(())
}