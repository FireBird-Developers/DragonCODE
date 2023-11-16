fn main(){
    let config = slint_build::CompilerConfiguration::new()
        .with_style("material-dark".to_owned());
    slint_build::compile_with_config("ui/MainWindow.slint", config).unwrap();
}