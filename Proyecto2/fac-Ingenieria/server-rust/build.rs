fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/cliente.proto")?;
    tonic_build::compile_protos("proto/disciplina.proto")?;
    Ok(())
}
