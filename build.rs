fn main(){
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material".into());
    slint_build::compile_with_config(r"src\ui\MainWindow.slint").unwrap();
}