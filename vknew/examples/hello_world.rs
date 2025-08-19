use wgpu::rwh::{HasDisplayHandle, HasWindowHandle};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

#[tokio::main]
async fn main() {
    let evnet_loop = EventLoop::builder().build().unwrap();

    let info = Some(vknew::wgpu::WasmCompatibilityCreateInfo::new().await);
    // let info = None;
    evnet_loop.run_app(&mut App::new(info)).unwrap();
}

struct App {
    window: Option<Window>,
    renderer: Option<Renderer>,
    info: Option<vknew::wgpu::WasmCompatibilityCreateInfo>,
}

impl App {
    pub fn new(info: Option<vknew::wgpu::WasmCompatibilityCreateInfo>) -> Self {
        Self {
            window: None,
            renderer: None,
            info,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_inner_size(PhysicalSize::new(640, 480))
            .with_resizable(false);
        let window = event_loop.create_window(window_attributes).unwrap();

        let info = self.info.take();
        self.renderer = Some(Renderer::new(&window, info));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(_) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &self.renderer {
                    renderer.render();
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

struct Renderer {
    instance: ash::Instance,
    device: ash::Device,
    queue: ash::vk::Queue,

    surface_loader: ash::khr::surface::Instance,
    surface: ash::vk::SurfaceKHR,
    swapchain_loader: ash::khr::swapchain::Device,
    swapchain: ash::vk::SwapchainKHR,
    dynamic_rendering_loader: ash::khr::dynamic_rendering::Device,
    images: Vec<ash::vk::Image>,
    image_views: Vec<ash::vk::ImageView>,
    display_semaphore: ash::vk::Semaphore,
    command_semaphore: ash::vk::Semaphore,

    command_pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,
    vs_shader_module: ash::vk::ShaderModule,
    fs_shader_module: ash::vk::ShaderModule,
    pipeline: ash::vk::Pipeline,
    layout: ash::vk::PipelineLayout,
    buffer: ash::vk::Buffer,
    device_memory: ash::vk::DeviceMemory,
}

impl Renderer {
    pub fn new<W>(window: W, mut info: Option<vknew::wgpu::WasmCompatibilityCreateInfo>) -> Self
    where
        W: HasWindowHandle + HasDisplayHandle,
    {
        let entry = if info.is_some() {
            unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) }
        } else {
            ash::Entry::linked()
        };

        let instance = {
            let app_info =
                ash::vk::ApplicationInfo::default().api_version(ash::vk::API_VERSION_1_3);
            let enabled_layer_names = [c"VK_LAYER_KHRONOS_validation".as_ptr()];
            let enabled_extension_names: Vec<_> = ash_window::enumerate_required_extensions(
                window.display_handle().unwrap().as_raw(),
            )
            .unwrap()
            .to_vec()
            .into_iter()
            .chain([
                ash::ext::debug_utils::NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::khr::portability_enumeration::NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::khr::get_physical_device_properties2::NAME.as_ptr(),
            ])
            .collect();
            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                ash::vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                ash::vk::InstanceCreateFlags::default()
            };
            let create_info = ash::vk::InstanceCreateInfo::default()
                .application_info(&app_info)
                .enabled_layer_names(&enabled_layer_names)
                .enabled_extension_names(&enabled_extension_names)
                .flags(create_flags);
            if let Some(info) = &mut info {
                unsafe { entry.create_instance(&create_info.push_next(info), None) }.unwrap()
            } else {
                unsafe { entry.create_instance(&create_info, None) }.unwrap()
            }
        };

        let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
        let device = {
            let priorities = [1.0];
            let queue_create_infos = [ash::vk::DeviceQueueCreateInfo::default()
                .queue_family_index(0)
                .queue_priorities(&priorities)];
            let enabled_extension_names = [
                ash::khr::swapchain::NAME.as_ptr(),
                ash::khr::dynamic_rendering::NAME.as_ptr(),
                ash::khr::synchronization2::NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::khr::portability_subset::NAME.as_ptr(),
            ];
            let mut extension =
                ash::vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);
            let mut sync_extension =
                ash::vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true);
            let create_info = ash::vk::DeviceCreateInfo::default()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&enabled_extension_names)
                .push_next(&mut extension)
                .push_next(&mut sync_extension);
            unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap()
        };
        let queue = unsafe {
            device.get_device_queue(0 /*queue_family_index*/, 0 /*queue_index*/)
        };

        let dynamic_rendering_loader = ash::khr::dynamic_rendering::Device::new(&instance, &device);

        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        let surface = if info.is_some() {
            vknew::wgpu::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .unwrap()
        } else {
            unsafe {
                ash_window::create_surface(
                    &entry,
                    &instance,
                    window.display_handle().unwrap().as_raw(),
                    window.window_handle().unwrap().as_raw(),
                    None,
                )
                .unwrap()
            }
        };

        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
        }
        .unwrap();
        let surface_format =
            unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface) }
                .unwrap()[0];

        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &device);
        let swapchain = {
            let create_info = ash::vk::SwapchainCreateInfoKHR::default()
                .surface(surface)
                .min_image_count(2)
                .image_format(surface_format.format)
                .image_color_space(surface_format.color_space)
                .pre_transform(surface_capabilities.supported_transforms)
                .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
                .image_extent(ash::vk::Extent2D::default().width(640).height(480))
                .image_array_layers(1)
                .image_usage(ash::vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(&[0])
                .present_mode(ash::vk::PresentModeKHR::FIFO);
            unsafe { swapchain_loader.create_swapchain(&create_info, None) }.unwrap()
        };

        let (vs_shader_module, fs_shader_module) = {
            let vs_source = r"
            #version 450

            layout (location = 0) in vec2 i_Position;
            
            void main()
            {
                gl_Position = vec4(i_Position, 0.0, 1.0);
            }";
            let fs_source = r"
            #version 450
            layout(location = 0) out vec4 o_Color;
            void main()
            {
                o_Color = vec4(1.0);
            }";

            let convert = |source: &str, stage: naga::ShaderStage| {
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
            };
            let vs_code = convert(&vs_source, naga::ShaderStage::Vertex);
            let fs_code = convert(&fs_source, naga::ShaderStage::Fragment);

            unsafe {
                (
                    device
                        .create_shader_module(
                            &ash::vk::ShaderModuleCreateInfo::default().code(&vs_code),
                            None,
                        )
                        .unwrap(),
                    device
                        .create_shader_module(
                            &ash::vk::ShaderModuleCreateInfo::default().code(&fs_code),
                            None,
                        )
                        .unwrap(),
                )
            }
        };

        let layout = {
            let create_info = ash::vk::PipelineLayoutCreateInfo::default();
            unsafe { device.create_pipeline_layout(&create_info, None) }.unwrap()
        };

        let pipeline = {
            let stages = [
                ash::vk::PipelineShaderStageCreateInfo::default()
                    .stage(ash::vk::ShaderStageFlags::VERTEX)
                    .module(vs_shader_module)
                    .name(c"main"),
                ash::vk::PipelineShaderStageCreateInfo::default()
                    .stage(ash::vk::ShaderStageFlags::FRAGMENT)
                    .module(fs_shader_module)
                    .name(c"main"),
            ];
            let vertex_attribute_descriptions =
                [ash::vk::VertexInputAttributeDescription::default()
                    .location(0)
                    .binding(0)
                    .offset(0)
                    .format(ash::vk::Format::R32G32_SFLOAT)];
            let vertex_binding_descriptions = [ash::vk::VertexInputBindingDescription::default()
                .stride(std::mem::size_of::<f32>() as u32 * 2)];
            let vertex_input_state = ash::vk::PipelineVertexInputStateCreateInfo::default()
                .vertex_attribute_descriptions(&vertex_attribute_descriptions)
                .vertex_binding_descriptions(&vertex_binding_descriptions);
            let input_assembly_state = ash::vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST);
            // 原点を Bottom-Left にしないと座標系が一致しない
            let viewports = [ash::vk::Viewport::default()
                .width(640.0)
                .height(-480.0)
                .x(0.0)
                .y(480.0)];
            let scissors = [ash::vk::Rect2D::default()
                .extent(ash::vk::Extent2D::default().width(640).height(480))];
            let viewport_state = ash::vk::PipelineViewportStateCreateInfo::default()
                .viewports(&viewports)
                .scissors(&scissors);
            let rasterization_state = ash::vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(ash::vk::PolygonMode::FILL)
                .line_width(1.0);
            let multisample_state = ash::vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1);
            let depth_stencil_state = ash::vk::PipelineDepthStencilStateCreateInfo::default();
            let attachments = [ash::vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(ash::vk::ColorComponentFlags::RGBA)];
            let color_blend_state = ash::vk::PipelineColorBlendStateCreateInfo::default()
                .logic_op(ash::vk::LogicOp::CLEAR)
                .attachments(&attachments);
            let dynamic_state = ash::vk::PipelineDynamicStateCreateInfo::default();
            let color_attachment_formatts = [surface_format.format];
            let mut pipeline_rendering_create_info =
                ash::vk::PipelineRenderingCreateInfo::default()
                    .color_attachment_formats(&color_attachment_formatts);
            let create_info = ash::vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input_state)
                .input_assembly_state(&input_assembly_state)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterization_state)
                .multisample_state(&multisample_state)
                .depth_stencil_state(&depth_stencil_state)
                .color_blend_state(&color_blend_state)
                .dynamic_state(&dynamic_state)
                .layout(layout)
                .render_pass(ash::vk::RenderPass::null())
                .push_next(&mut pipeline_rendering_create_info);
            unsafe {
                device.create_graphics_pipelines(
                    ash::vk::PipelineCache::null(),
                    &[create_info],
                    None,
                )
            }
            .unwrap()
        }[0];

        let buffer = {
            let create_info = ash::vk::BufferCreateInfo::default()
                .size(64)
                .usage(ash::vk::BufferUsageFlags::VERTEX_BUFFER)
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
                .allocation_size(64)
                .memory_type_index(memory_type_index);
            unsafe { device.allocate_memory(&create_info, None) }.unwrap()
        };

        {
            let ptr = unsafe {
                device.map_memory(device_memory, 0, 64, ash::vk::MemoryMapFlags::empty())
            }
            .unwrap();
            let vertex_data = [0.0, 0.5, -0.5, -0.5, 0.5, -0.5];
            let storage = unsafe { std::slice::from_raw_parts_mut(ptr as *mut f32, 6) };
            storage[0..vertex_data.len()].copy_from_slice(&vertex_data);

            unsafe { device.unmap_memory(device_memory) };
        }

        unsafe { device.bind_buffer_memory(buffer, device_memory, 0) }.unwrap();

        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };
        let image_views: Vec<_> = images
            .iter()
            .map(|image| {
                let create_info = ash::vk::ImageViewCreateInfo::default()
                    .image(*image)
                    .format(surface_format.format)
                    .view_type(ash::vk::ImageViewType::TYPE_2D)
                    .subresource_range(
                        ash::vk::ImageSubresourceRange::default()
                            .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .layer_count(1)
                            .level_count(1),
                    );
                unsafe { device.create_image_view(&create_info, None) }.unwrap()
            })
            .collect();

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

        let (display_semaphore, command_semaphore) = {
            let create_info = ash::vk::SemaphoreCreateInfo::default();
            unsafe {
                (
                    device.create_semaphore(&create_info, None).unwrap(),
                    device.create_semaphore(&create_info, None).unwrap(),
                )
            }
        };

        Self {
            instance,
            device,
            queue,
            command_pool,
            command_buffer,
            surface_loader,
            surface,
            swapchain_loader,
            swapchain,
            dynamic_rendering_loader,
            images,
            image_views,
            display_semaphore,
            command_semaphore,
            vs_shader_module,
            fs_shader_module,
            pipeline,
            layout,
            buffer,
            device_memory,
        }
    }

    pub fn render(&self) {
        let device = &self.device;

        // unsafe { device.reset_fences(&[self.display_fence]) }.unwrap();

        let (next_image_index, _) = unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.display_semaphore,
                ash::vk::Fence::null(),
            )
        }
        .unwrap();

        unsafe {
            device.reset_command_buffer(
                self.command_buffer,
                ash::vk::CommandBufferResetFlags::empty(),
            )
        }
        .unwrap();

        let begin_info = ash::vk::CommandBufferBeginInfo::default();
        unsafe { device.begin_command_buffer(self.command_buffer, &begin_info) }.unwrap();

        unsafe {
            device.cmd_pipeline_barrier(
                self.command_buffer,
                ash::vk::PipelineStageFlags::TOP_OF_PIPE,
                ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                ash::vk::DependencyFlags::empty(),
                &[],
                &[],
                &[ash::vk::ImageMemoryBarrier::default()
                    .old_layout(ash::vk::ImageLayout::UNDEFINED)
                    .new_layout(ash::vk::ImageLayout::ATTACHMENT_OPTIMAL)
                    .image(self.images[next_image_index as usize])
                    .subresource_range(
                        ash::vk::ImageSubresourceRange::default()
                            .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1),
                    )
                    .dst_access_mask(ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE)],
            )
        };

        let color_attachments = [ash::vk::RenderingAttachmentInfo::default()
            .image_view(self.image_views[next_image_index as usize])
            .image_layout(ash::vk::ImageLayout::ATTACHMENT_OPTIMAL)
            .load_op(ash::vk::AttachmentLoadOp::CLEAR)
            .clear_value(ash::vk::ClearValue {
                color: ash::vk::ClearColorValue {
                    float32: [0.1, 0.2, 0.3, 1.0],
                },
            })];
        let rendering_info = ash::vk::RenderingInfo::default()
            .render_area(
                ash::vk::Rect2D::default()
                    .extent(ash::vk::Extent2D::default().width(640).height(480)),
            )
            .layer_count(1)
            .color_attachments(&color_attachments);
        unsafe {
            self.dynamic_rendering_loader
                .cmd_begin_rendering(self.command_buffer, &rendering_info)
        };

        unsafe {
            device.cmd_bind_pipeline(
                self.command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            )
        };

        unsafe {
            device.cmd_bind_vertex_buffers(
                self.command_buffer,
                0, /*first_binding*/
                &[self.buffer],
                &[0],
            )
        };

        unsafe {
            device.cmd_draw(
                self.command_buffer,
                3, /*vertex_count*/
                1, /*instance_count*/
                0, /*first_vertex*/
                0, /*first_instance*/
            )
        };

        unsafe {
            self.dynamic_rendering_loader
                .cmd_end_rendering(self.command_buffer)
        };

        unsafe {
            device.cmd_pipeline_barrier(
                self.command_buffer,
                ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                ash::vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                ash::vk::DependencyFlags::empty(),
                &[],
                &[],
                &[ash::vk::ImageMemoryBarrier::default()
                    .old_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .new_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR)
                    .image(self.images[next_image_index as usize])
                    .subresource_range(
                        ash::vk::ImageSubresourceRange::default()
                            .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                            .base_mip_level(0)
                            .level_count(1)
                            .base_array_layer(0)
                            .layer_count(1),
                    )
                    .src_access_mask(ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                    .dst_access_mask(ash::vk::AccessFlags::MEMORY_READ)],
            )
        };

        unsafe { device.end_command_buffer(self.command_buffer) }.unwrap();

        let wait_semaphores = [self.display_semaphore];
        let signal_semaphores = [self.command_semaphore];
        let command_buffers = [self.command_buffer];
        let wait_dst_stage_mask = [ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let submits = [ash::vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .signal_semaphores(&signal_semaphores)
            .command_buffers(&command_buffers)
            .wait_dst_stage_mask(&wait_dst_stage_mask)];
        unsafe { device.queue_submit(self.queue, &submits, ash::vk::Fence::null()) }.unwrap();

        let wait_semaphores = [];
        let swapchains = [self.swapchain];
        let image_indices = [next_image_index];
        let present_info = ash::vk::PresentInfoKHR::default()
            .wait_semaphores(&wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        unsafe {
            self.swapchain_loader
                .queue_present(self.queue, &present_info)
        }
        .unwrap();

        unsafe { device.device_wait_idle() }.unwrap();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        let device = &self.device;

        unsafe {
            device.device_wait_idle().unwrap();

            device.free_memory(self.device_memory, None);
            device.destroy_buffer(self.buffer, None);

            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_pipeline(self.pipeline, None);

            device.destroy_shader_module(self.vs_shader_module, None);
            device.destroy_shader_module(self.fs_shader_module, None);

            device.destroy_semaphore(self.display_semaphore, None);
            device.destroy_semaphore(self.command_semaphore, None);

            device.free_command_buffers(self.command_pool, &[self.command_buffer]);
            device.destroy_command_pool(self.command_pool, None);

            for image_view in &self.image_views {
                device.destroy_image_view(*image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.surface_loader.destroy_surface(self.surface, None);

            self.queue = ash::vk::Queue::null();
            device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
