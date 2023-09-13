use dek::compiler::Compiler;

fn main() -> miette::Result<()> {
    miette::set_panic_hook();

    let mut compiler = Compiler::new();
    compiler.compile("main.dek")?;

    Ok(())
}
