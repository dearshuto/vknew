#[test]
fn entry() {
    let _entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };
}

#[test]
fn instance() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let create_info = ash::vk::InstanceCreateInfo::default();
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };
    unsafe { instance.destroy_instance(None) };
}

#[tokio::test]
async fn physical_devices() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let mut extension = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut extension);
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };

    let physical_devices = unsafe { instance.enumerate_physical_devices() }.unwrap();
    assert_eq!(physical_devices.len(), 1);

    unsafe { instance.destroy_instance(None) };
}

#[tokio::test]
async fn device() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let mut extension = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut extension);
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];

    let create_info = ash::vk::DeviceCreateInfo::default();
    let device = unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap();

    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}

#[tokio::test]
async fn queue() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let mut extension = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut extension);
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
    let create_info = ash::vk::DeviceCreateInfo::default();
    let device = unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap();

    let _queue = unsafe {
        device.get_device_queue(0 /*queue_family_index*/, 0)
    };

    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}

#[tokio::test]
async fn command_pool() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let mut extension = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut extension);
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
    let create_info = ash::vk::DeviceCreateInfo::default();
    let device = unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap();

    let command_pool = {
        let create_info = ash::vk::CommandPoolCreateInfo::default();
        unsafe { device.create_command_pool(&create_info, None) }.unwrap()
    };

    unsafe { device.destroy_command_pool(command_pool, None) };
    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}

#[tokio::test]
async fn command_buffer() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let mut extension = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut extension);
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
    let create_info = ash::vk::DeviceCreateInfo::default();
    let device = unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap();

    let command_pool = {
        let create_info = ash::vk::CommandPoolCreateInfo::default();
        unsafe { device.create_command_pool(&create_info, None) }.unwrap()
    };

    let command_buffers = {
        let allocate_info = ash::vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(2);
        unsafe { device.allocate_command_buffers(&allocate_info) }.unwrap()
    };
    assert_eq!(command_buffers.len(), 2);

    unsafe { device.destroy_command_pool(command_pool, None) };
    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}

#[tokio::test]
async fn shader_module() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };

    let instance = {
        let mut extension = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut extension);
        unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
    let create_info = ash::vk::DeviceCreateInfo::default();
    let device = unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap();

    let source = r"
        #version 450
        void main()
        {
            gl_Position = vec4(1.0);
        }
    ";
    let options = naga::front::glsl::Options::from(naga::ShaderStage::Vertex);
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
    let code = naga::back::spv::write_vec(&module, &info, &options, None).unwrap();

    let shader_module = {
        let create_info = ash::vk::ShaderModuleCreateInfo::default().code(&code);
        unsafe { device.create_shader_module(&create_info, None) }.unwrap()
    };

    unsafe { device.destroy_shader_module(shader_module, None) };
    unsafe { device.destroy_device(None) };
    unsafe { instance.destroy_instance(None) };
}
