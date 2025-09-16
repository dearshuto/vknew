#[path = "common/mod.rs"]
mod common;

#[tokio::main]
async fn main() {
    let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };
    // let entry = ash::Entry::linked();

    let instance = {
        let app_info = ash::vk::ApplicationInfo::default().api_version(ash::vk::API_VERSION_1_3);
        let enabled_layer_names = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
        let enabled_extension_names = [
            ash::ext::debug_utils::NAME.as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            ash::khr::portability_enumeration::NAME.as_ptr(),
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            ash::khr::get_physical_device_properties2::NAME.as_ptr(),
        ];
        let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            ash::vk::InstanceCreateFlags::default()
        };
        let mut info = vknew::wgpu::WasmCompatibilityCreateInfo::new().await;
        let create_info = ash::vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_layer_names(&enabled_layer_names)
            .enabled_extension_names(&enabled_extension_names)
            .flags(create_flags);
        unsafe { entry.create_instance(&create_info.push_next(&mut info), None) }.unwrap()
        // unsafe { entry.create_instance(&create_info, None) }.unwrap()
    };
    let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
    let device = {
        let priorities = [1.0];
        let queue_create_infos = [ash::vk::DeviceQueueCreateInfo::default()
            .queue_family_index(0)
            .queue_priorities(&priorities)];
        let enabled_extension_names = [
            #[cfg(any(target_os = "macos", target_os = "ios"))]
            ash::khr::portability_subset::NAME.as_ptr(),
        ];
        let create_info = ash::vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&enabled_extension_names);
        unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap()
    };

    let queue = unsafe {
        device.get_device_queue(0 /*queue_family_index*/, 0 /*queue_index*/)
    };

    let shader_module = {
        let source = r"
            #version 450
            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

            layout(binding = 0) buffer Storage {
                uint u_Data[];  
            };

            void main()
            {
                uint index = gl_GlobalInvocationID.x;
                u_Data[index] = index;
            }
        ";
        let code = common::convert_shader(&source, naga::ShaderStage::Compute);
        let create_info = ash::vk::ShaderModuleCreateInfo::default().code(&code);
        unsafe { device.create_shader_module(&create_info, None) }.unwrap()
    };

    let layout = {
        let bindings = [ash::vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .stage_flags(ash::vk::ShaderStageFlags::COMPUTE)
            .descriptor_type(ash::vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)];
        let create_info = ash::vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        unsafe { device.create_descriptor_set_layout(&create_info, None) }.unwrap()
    };
    let pipeline_layout = {
        let set_layouts = [layout];
        let create_info = ash::vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts);
        unsafe { device.create_pipeline_layout(&create_info, None) }.unwrap()
    };

    let pipeline = {
        let create_info = ash::vk::ComputePipelineCreateInfo::default()
            .stage(
                ash::vk::PipelineShaderStageCreateInfo::default()
                    .stage(ash::vk::ShaderStageFlags::COMPUTE)
                    .module(shader_module)
                    .name(c"main"),
            )
            .layout(pipeline_layout);
        unsafe {
            device.create_compute_pipelines(ash::vk::PipelineCache::null(), &[create_info], None)
        }
        .unwrap()[0]
    };

    let buffer = {
        let create_info = ash::vk::BufferCreateInfo::default()
            .size(1024)
            .usage(ash::vk::BufferUsageFlags::STORAGE_BUFFER)
            .queue_family_indices(&[0]);
        unsafe { device.create_buffer(&create_info, None) }.unwrap()
    };

    let device_memory = {
        let device_memory_properties =
            unsafe { instance.get_physical_device_memory_properties(physical_device) };
        let requirement = unsafe { device.get_buffer_memory_requirements(buffer) };
        let flags = ash::vk::MemoryPropertyFlags::HOST_VISIBLE
            | ash::vk::MemoryPropertyFlags::HOST_COHERENT;
        let memory_type_index = device_memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, memory_type)| {
                (1 << index) & requirement.memory_type_bits != 0
                    && memory_type.property_flags & flags == flags
            })
            .map(|(index, _)| index as u32)
            .unwrap();
        let create_info = ash::vk::MemoryAllocateInfo::default()
            .allocation_size(1024)
            .memory_type_index(memory_type_index);
        unsafe { device.allocate_memory(&create_info, None) }.unwrap()
    };

    unsafe { device.bind_buffer_memory(buffer, device_memory, 0) }.unwrap();

    let descriptor_pool = {
        let pool_sizes = [ash::vk::DescriptorPoolSize::default()
            .descriptor_count(1)
            .ty(ash::vk::DescriptorType::STORAGE_BUFFER)];
        let create_info = ash::vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(1);
        unsafe { device.create_descriptor_pool(&create_info, None) }.unwrap()
    };

    let descriptor_sets = {
        let set_layouts = [layout];
        let create_info = ash::vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&set_layouts);
        unsafe { device.allocate_descriptor_sets(&create_info) }.unwrap()
    };

    let descriptor_buffer_info = [ash::vk::DescriptorBufferInfo::default()
        .buffer(buffer)
        .offset(0)
        .range(1024)];
    let descriptor_writes = [ash::vk::WriteDescriptorSet::default()
        .dst_set(descriptor_sets[0])
        .dst_binding(0)
        .descriptor_count(1)
        .descriptor_type(ash::vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&descriptor_buffer_info)];
    unsafe { device.update_descriptor_sets(&descriptor_writes, &[]) };

    let command_pool = {
        let create_info = ash::vk::CommandPoolCreateInfo::default()
            .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        unsafe { device.create_command_pool(&create_info, None) }.unwrap()
    };
    let command_buffer = {
        let allocate_info = ash::vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        unsafe { device.allocate_command_buffers(&allocate_info) }.unwrap()
    }[0];

    unsafe {
        device.reset_command_buffer(command_buffer, ash::vk::CommandBufferResetFlags::empty())
    }
    .unwrap();

    let begin_info = ash::vk::CommandBufferBeginInfo::default();
    unsafe { device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();
    unsafe {
        device.cmd_bind_pipeline(
            command_buffer,
            ash::vk::PipelineBindPoint::COMPUTE,
            pipeline,
        )
    };
    unsafe {
        device.cmd_bind_descriptor_sets(
            command_buffer,
            ash::vk::PipelineBindPoint::COMPUTE,
            pipeline_layout,
            0,
            &descriptor_sets,
            &[],
        )
    };
    unsafe { device.cmd_dispatch(command_buffer, 64, 1, 1) };
    unsafe { device.end_command_buffer(command_buffer) }.unwrap();

    let command_buffers = [command_buffer];
    let submits = [ash::vk::SubmitInfo::default().command_buffers(&command_buffers)];
    unsafe { device.queue_submit(queue, &submits, ash::vk::Fence::null()) }.unwrap();
    unsafe { device.device_wait_idle().unwrap() }

    unsafe {
        device.free_command_buffers(command_pool, &[command_buffer]);
        device.destroy_command_pool(command_pool, None);
        device.free_memory(device_memory, None);
        // device
        //     .free_descriptor_sets(descriptor_pool, &descriptor_sets)
        //     .unwrap();
        device.destroy_descriptor_pool(descriptor_pool, None);
        device.destroy_buffer(buffer, None);
        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_descriptor_set_layout(layout, None);
        device.destroy_shader_module(shader_module, None);
        device.destroy_device(None);
        instance.destroy_instance(None);
    }
}
