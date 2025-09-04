pub fn convert_shader(source: &str, stage: naga::ShaderStage) -> Vec<u32> {
    let options = naga::front::glsl::Options::from(stage);
    let module = naga::front::glsl::Frontend::default()
        .parse(&options, &source)
        .unwrap();
    let options = naga::back::spv::Options::default();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();
    naga::back::spv::write_vec(&module, &info, &options, None).unwrap()
}
