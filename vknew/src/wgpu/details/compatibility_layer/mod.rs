mod command;
mod id_generator;
mod type_converter;

use command::{Command, CommandEmulator, DrawParameters};
use id_generator::IdGenerator;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use std::collections::HashSet;
use std::ffi::c_void;
use std::{borrow::Cow, collections::HashMap, ffi::CStr, time::Duration};

use ash::vk::{Handle, TaggedStructure};

use crate::wgpu::details::compatibility_layer::command::BeginRenderingParameters;
use crate::wgpu::details::compatibility_layer::type_converter::{
    CompositeAlpha, TextureFormat, TextureUsage,
};
use crate::wgpu::{ExtensionCreateInfoBase, WasmCompatibilityCreateInfo};

pub struct CompatibilityLayer;

impl CompatibilityLayer {
    pub fn create_instance(
        create_info: &ash::vk::InstanceCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_instance: &mut ash::vk::Instance,
    ) -> ash::vk::Result {
        let mut extension_ptr = create_info.p_next;
        while extension_ptr != std::ptr::null() {
            let extension = unsafe { &*(extension_ptr as *const ExtensionCreateInfoBase) };
            if extension.s_type == WasmCompatibilityCreateInfo::STRUCTURE_TYPE {
                let extension = unsafe { &*(extension_ptr as *const WasmCompatibilityCreateInfo) };
                let Some(instance) = &extension.instance else {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };
                let Some(device) = &extension.device else {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };
                let Some(queue) = &extension.queue else {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };
                let Some(adapter) = &extension.adapter else {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };

                AccessorMut::from(dst_instance).allocate(InstanceContext {
                    instance: instance.clone(),
                    device: device.clone(),
                    queue: queue.clone(),
                    adapter: adapter.clone(),
                    surface_table: Default::default(),
                    surface_id_generator: IdGenerator::new(),
                    context: None,
                });
                break;
            }

            extension_ptr = extension.p_next;
        }
        ash::vk::Result::SUCCESS
    }

    pub fn destroy_instance(
        mut instance: ash::vk::Instance,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        AccessorMut::from(&mut instance).free();
    }

    pub fn enumerate_physical_devices(
        instance: ash::vk::Instance,
        dst_physical_devices: &mut [ash::vk::PhysicalDevice],
    ) -> ash::vk::Result {
        for physical_device in dst_physical_devices {
            AccessorMut::from(physical_device).with(InstanceHandleAdapter(instance));
        }

        ash::vk::Result::SUCCESS
    }

    pub fn create_surface(
        _entry: &ash::Entry,
        instance: &ash::Instance,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
        _allocation_callbacks: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
        let mut handle: ash::vk::Instance = instance.handle();
        AccessorMut::from(&mut handle).update(|context: &mut _| {
            let surface = unsafe {
                context
                    .instance
                    .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                        raw_display_handle: display_handle,
                        raw_window_handle: window_handle,
                    })
            };
            let Ok(surface) = surface else {
                return Err(ash::vk::Result::ERROR_UNKNOWN);
            };

            let device = &context.device;
            let adapter = &context.adapter;
            let config = surface.get_default_config(adapter, 640, 480).unwrap();
            surface.configure(device, &config);

            let id = context.surface_id_generator.generate();
            context.surface_table.insert(id, surface);

            Ok(ash::vk::SurfaceKHR::from_raw(id.into()))
        })
    }

    pub fn destroy_surface(
        mut instance: ash::vk::Instance,
        surface: ash::vk::SurfaceKHR,
        _allocation_callbacks: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let id = Accessor::from(surface).peek();
        AccessorMut::from(&mut instance).update(|context: &mut _| {
            context.surface_table.remove(&id);
        });
    }

    pub fn get_physical_device_surface_capabilities_khr(
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
        dst_surface_capabilities: &mut ash::vk::SurfaceCapabilitiesKHR,
    ) -> ash::vk::Result {
        if physical_device.is_null() {
            return ash::vk::Result::ERROR_UNKNOWN;
        }

        if surface.is_null() {
            return ash::vk::Result::ERROR_UNKNOWN;
        }

        let id = Accessor::from(surface).peek();

        let instance_handle = Accessor::from(physical_device).peek().0;
        Accessor::from(instance_handle).peek_return(|context: &_| {
            let Some(surface) = context.surface_table.get(&id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let adapter = &context.adapter;
            let capabilities = surface.get_capabilities(adapter);

            *dst_surface_capabilities = ash::vk::SurfaceCapabilitiesKHR::default()
                .min_image_count(1)
                .max_image_count(1)
                .supported_composite_alpha(CompositeAlpha::to_vk(&capabilities.alpha_modes))
                .supported_usage_flags(TextureUsage::to_vk(capabilities.usages));
            ash::vk::Result::SUCCESS
        })
    }

    pub fn get_physical_device_surface_formats_khr(
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
        dst_surface_formats: &mut [ash::vk::SurfaceFormatKHR],
    ) -> ash::vk::Result {
        let surface_id = Accessor::from(surface).peek();
        let instance_handle = Accessor::from(physical_device).peek().0;
        let formats = Accessor::from(instance_handle).peek_return(|context: &_| {
            let Some(surface) = context.surface_table.get(&surface_id) else {
                return Default::default();
            };

            let capabilities = surface.get_capabilities(&context.adapter);
            capabilities.present_modes;
            capabilities.formats
        });

        for index in 0..dst_surface_formats.len() {
            let format = formats[index];
            dst_surface_formats[index] = ash::vk::SurfaceFormatKHR::default()
                .format(TextureFormat::to_vk(format))
                .color_space(ash::vk::ColorSpaceKHR::SRGB_NONLINEAR);
        }

        ash::vk::Result::SUCCESS
    }

    pub fn create_device(
        physical_device: ash::vk::PhysicalDevice,
        _create_info: &ash::vk::DeviceCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_device: &mut ash::vk::Device,
    ) -> ash::vk::Result {
        let mut instance_handle_adapter = Accessor::from(physical_device).peek();

        // デバイスの書き込み
        // TODO：常に書き込むのではなく作成に成功したときのみ処理するようにしたい
        AccessorMut::from(dst_device).with(instance_handle_adapter);

        AccessorMut::from(&mut instance_handle_adapter.0).update(
            |instance_context: &mut InstanceContext<'_>| {
                if instance_context.context.is_some() {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };

                let context = Context {
                    command_pool_table: Default::default(),
                    swapchain_image_table: HashMap::default(),
                    image_view_table: HashMap::default(),
                    image_surface_table: HashMap::default(),
                    render_pipeline_table: HashMap::default(),
                    compute_pipeline_table: HashMap::default(),
                    shader_module_table: HashMap::default(),
                    semaphore_table: HashSet::default(),
                    pipeline_layout_table: Default::default(),
                    render_pipeline_id_generator: IdGenerator::new(),
                    command_pool_id_generator: IdGenerator::new(),
                    command_table: HashMap::default(),
                    shader_module_id_generator: IdGenerator::new(),
                    semaphore_id_generator: IdGenerator::new(),
                    command_buffer_id_generator: IdGenerator::new(),
                    image_id_generator: IdGenerator::new(),
                    image_view_id_generator: IdGenerator::new(),
                    pipeline_layout_id_generator: IdGenerator::new(),
                    swapchain_id_generator: IdGenerator::new(),
                    surface_texture: None,
                };
                instance_context.context = Some(context);

                ash::vk::Result::SUCCESS
            },
        )
    }

    pub fn destroy_device(
        mut device: ash::vk::Device,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        AccessorMut::from(&mut device).with(InstanceHandleAdapter(ash::vk::Instance::null()));
    }

    pub fn get_device_queue(
        device: ash::vk::Device,
        queue_family_index: u32,
        queue_index: u32,
        queue: &mut ash::vk::Queue,
    ) {
        if !(queue_family_index == 0 && queue_index == 0) {
            *queue = ash::vk::Queue::null();
            return;
        }

        let instance_handle = Accessor::from(device).peek().0;
        *queue = ash::vk::Queue::from_raw(InstanceHandleAdapter(instance_handle).into());
    }

    pub fn create_swapchain_khr(
        device: ash::vk::Device,
        create_info: &ash::vk::SwapchainCreateInfoKHR<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        swapchain: &mut ash::vk::SwapchainKHR,
    ) -> ash::vk::Result {
        let mut instance_handle: ash::vk::Instance = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let swapchain_id = context.swapchain_id_generator.generate();
            let image_id = context.image_id_generator.generate();
            context
                .swapchain_image_table
                .insert(swapchain_id, vec![image_id]);

            let surface_id = Accessor::from(create_info.surface).peek();
            context.image_surface_table.insert(image_id, surface_id);

            AccessorMut::from(swapchain).with(swapchain_id);
            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_swapchain_khr(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let id = Accessor::from(swapchain).peek();
            context.swapchain_image_table.remove(&id);
        });
    }

    pub fn get_swapchain_images_khr(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        dst_images: &mut [ash::vk::Image],
    ) -> ash::vk::Result {
        let instance_handle = Accessor::from(device).peek().0;
        Accessor::from(instance_handle).peek_return(|context: &_| {
            let Some(context) = &context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let swapchain_id = Accessor::from(swapchain).peek();
            let Some(images) = context.swapchain_image_table.get(&swapchain_id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            // 出力バッファーが足りなかった
            if dst_images.len() < images.len() {
                return ash::vk::Result::INCOMPLETE;
            }

            // 出力バッファーにコピー
            for index in 0..dst_images.len() {
                dst_images[index] = ash::vk::Image::from_raw(images[index].into());
            }

            ash::vk::Result::SUCCESS
        })
    }

    pub fn get_swapchain_image_count(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        out_count: &mut u32,
    ) -> ash::vk::Result {
        let _acessor = Accessor::from(device);
        let _swapchain_id = Accessor::from(swapchain).peek();
        *out_count = 1;
        ash::vk::Result::SUCCESS
    }

    pub fn acquire_next_image_khr(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        _timeout: Duration,
        _semaphore: ash::vk::Semaphore,
        _fence: ash::vk::Fence,
        out_image_index: &mut u32,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(device_context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let swapchain_id = Accessor::from(swapchain).peek();

            let Some(image_id) = device_context.swapchain_image_table.get(&swapchain_id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Some(surface_id) = device_context.image_surface_table.get(&image_id[0]) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Some(surface) = context.surface_table.get(surface_id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Ok(texture) = surface.get_current_texture() else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            device_context.surface_texture = Some(texture);
            *out_image_index = 0;
            ash::vk::Result::SUCCESS
        })
    }

    pub fn create_command_pool(
        device: ash::vk::Device,
        _create_info: &ash::vk::CommandPoolCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        command_pool: &mut ash::vk::CommandPool,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let id = context.command_pool_id_generator.generate();
            context.command_pool_table.insert(id, Default::default());

            // CommandPool ハンドルは識別子として利用する
            AccessorMut::from(command_pool).with(id);

            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_command_pool(
        device: ash::vk::Device,
        command_pool: ash::vk::CommandPool,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let id = Accessor::from(command_pool).peek();
            context.command_pool_table.remove(&id);
        });

        ash::vk::Result::SUCCESS
    }

    pub fn allocate_command_buffers(
        device: ash::vk::Device,
        allocate_info: &ash::vk::CommandBufferAllocateInfo<'_>,
        command_buffers: &mut [ash::vk::CommandBuffer],
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        let instance_handle_for_context = instance_handle;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(device_context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let pool_id = Accessor::from(allocate_info.command_pool).peek();

            for index in 0..command_buffers.len() {
                let command_buffer_id = device_context.command_buffer_id_generator.generate();
                let Some(list) = device_context.command_pool_table.get_mut(&pool_id) else {
                    continue;
                };

                list.insert(command_buffer_id);
                device_context
                    .command_table
                    .insert(command_buffer_id, Vec::default());
                AccessorMut::from(&mut command_buffers[index]).allocate(CommandBufferContext {
                    id: command_buffer_id,
                    instance_handle: instance_handle_for_context,
                });
            }

            ash::vk::Result::SUCCESS
        })
    }

    pub fn free_command_buffers(
        device: ash::vk::Device,
        command_pool: ash::vk::CommandPool,
        command_buffers: &[ash::vk::CommandBuffer],
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };
            for command_buffer in command_buffers {
                let mut command_buffer = *command_buffer;
                AccessorMut::from(&mut command_buffer).free();
            }

            let pool_id = Accessor::from(command_pool).peek();
            context.command_pool_table.remove(&pool_id);
            ash::vk::Result::SUCCESS
        })
    }

    pub fn create_shader_module(
        device: ash::vk::Device,
        create_info: &ash::vk::ShaderModuleCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_shader_module: &mut ash::vk::ShaderModule,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(device_context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let device = &context.device;
            let code = unsafe {
                std::slice::from_raw_parts(create_info.p_code, create_info.code_size / 4)
            };

            let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::SpirV(Cow::Borrowed(&code)),
            });

            let id = device_context.shader_module_id_generator.generate();
            device_context.shader_module_table.insert(id, shader_module);
            *dst_shader_module = ash::vk::ShaderModule::from_raw(id.into());

            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_shader_module(
        device: ash::vk::Device,
        shader_module: ash::vk::ShaderModule,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let id = Accessor::from(shader_module).peek();
            context.shader_module_table.remove(&id);
        });
    }

    pub fn create_pipeline_layout(
        device: ash::vk::Device,
        _create_info: &ash::vk::PipelineLayoutCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_pipeline_layout: &mut ash::vk::PipelineLayout,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let device = &context.device;
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

            let id = context.pipeline_layout_id_generator.generate();
            context.pipeline_layout_table.insert(id, layout);
            AccessorMut::from(dst_pipeline_layout).with(id);
            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_pipeline_layout(
        device: ash::vk::Device,
        pipeline_layout: ash::vk::PipelineLayout,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let id = Accessor::from(pipeline_layout).peek();
            context.pipeline_layout_table.remove(&id);
        });
    }

    pub fn create_graphics_pipelines(
        device: ash::vk::Device,
        _pipeline_cache: ash::vk::PipelineCache,
        infos: &[ash::vk::GraphicsPipelineCreateInfo<'_>],
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_pipelines: &mut [ash::vk::Pipeline],
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let device = &context.device;
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            for (index, info) in infos.iter().enumerate() {
                let _input_assembly_state = unsafe { &*info.p_input_assembly_state };
                let shader_stages =
                    unsafe { std::slice::from_raw_parts(info.p_stages, info.stage_count as usize) };
                let vs_id = Accessor::from(shader_stages[0].module).peek();
                let fs_id = Accessor::from(shader_stages[1].module).peek();
                let Some(vs_module) = context.shader_module_table.get(&vs_id) else {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };
                let Some(fs_module) = context.shader_module_table.get(&fs_id) else {
                    return ash::vk::Result::ERROR_UNKNOWN;
                };

                let render_pipeline =
                    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: None,
                        layout: None,
                        vertex: wgpu::VertexState {
                            module: vs_module,
                            entry_point: Some(unsafe {
                                CStr::from_ptr(shader_stages[0].p_name).to_str().unwrap()
                            }),
                            compilation_options: wgpu::PipelineCompilationOptions::default(),
                            buffers: &[],
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: fs_module,
                            entry_point: Some(unsafe {
                                CStr::from_ptr(shader_stages[1].p_name).to_str().unwrap()
                            }),
                            compilation_options: wgpu::PipelineCompilationOptions::default(),
                            targets: &[Some(wgpu::ColorTargetState {
                                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                                blend: None,
                                write_mask: wgpu::ColorWrites::all(),
                            })],
                        }),
                        primitive: wgpu::PrimitiveState::default(),
                        depth_stencil: None,
                        multisample: wgpu::MultisampleState::default(),
                        multiview: None,
                        cache: None,
                    });
                let id = context.render_pipeline_id_generator.generate();
                context.render_pipeline_table.insert(id, render_pipeline);
                AccessorMut::from(&mut dst_pipelines[index]).with(id);
            }

            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_pipeline(
        device: ash::vk::Device,
        pipeline: ash::vk::Pipeline,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let id = Accessor::from(pipeline).peek();
            context.render_pipeline_table.remove(&id);
        });
    }

    pub fn create_image_view(
        device: ash::vk::Device,
        create_info: &ash::vk::ImageViewCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_view: &mut ash::vk::ImageView,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            // ハンドルへの id の書き込み
            let image_view_id = context.image_view_id_generator.generate();
            AccessorMut::from(dst_view).with(image_view_id);
            // Image との紐付け
            let image_id = Accessor::from(create_info.image).peek();
            context.image_view_table.insert(image_view_id, image_id);

            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_image_view(
        device: ash::vk::Device,
        image_view: ash::vk::ImageView,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut InstanceContext| {
            let id = Accessor::from(image_view).peek();
            let Some(context) = &mut context.context else {
                return;
            };
            context.image_view_table.remove(&id);
        });
    }

    pub fn create_semaphore(
        device: ash::vk::Device,
        _create_info: &ash::vk::SemaphoreCreateInfo<'_>,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
        dst_semaphore: &mut ash::vk::Semaphore,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(device).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut InstanceContext| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let id = context.semaphore_id_generator.generate();
            AccessorMut::from(dst_semaphore).with(id);
            context.semaphore_table.insert(id);

            ash::vk::Result::SUCCESS
        })
    }

    pub fn destroy_semaphore(
        device: ash::vk::Device,
        semaphore: ash::vk::Semaphore,
        _allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
    ) {
        let mut instance_handle = Accessor::from(device).peek().0;
        let id = Accessor::from(semaphore).peek();
        AccessorMut::from(&mut instance_handle).update(|context: &mut InstanceContext| {
            let Some(context) = &mut context.context else {
                return;
            };

            context.semaphore_table.remove(&id);
        });
    }

    pub fn reset_command_buffer(
        command_buffer: ash::vk::CommandBuffer,
        _flags: ash::vk::CommandBufferResetFlags,
    ) -> ash::vk::Result {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            command_list.clear();

            ash::vk::Result::SUCCESS
        })
    }

    pub fn begin_command_buffer(
        command_buffer: ash::vk::CommandBuffer,
        _begin_info: &ash::vk::CommandBufferBeginInfo<'_>,
    ) -> ash::vk::Result {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));

        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            command_list.push(Command::Begin);
            ash::vk::Result::SUCCESS
        })
    }

    pub fn end_command_buffer(
        command_buffer: ash::vk::CommandBuffer,
        _flags: ash::vk::CommandBufferResetFlags,
    ) -> ash::vk::Result {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));

        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            command_list.push(Command::End);

            ash::vk::Result::SUCCESS
        })
    }

    pub fn cmd_begin_rendering(
        command_buffer: ash::vk::CommandBuffer,
        info: &ash::vk::RenderingInfo<'_>,
    ) {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return;
            };

            let color_attachments = unsafe {
                std::slice::from_raw_parts(
                    info.p_color_attachments,
                    info.color_attachment_count as usize,
                )
            };
            let color_attachment = color_attachments[0];
            let clear = unsafe { color_attachment.clear_value.color.float32 };
            let parameters = BeginRenderingParameters { clear };
            command_list.push(Command::BeginRendering(parameters));
        });
    }

    pub fn cmd_end_rendering(command_buffer: ash::vk::CommandBuffer) {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));

        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return;
            };

            command_list.push(Command::EndRendering);
        })
    }

    pub fn cmd_bind_pipeline(
        command_buffer: ash::vk::CommandBuffer,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        pipeline: ash::vk::Pipeline,
    ) {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));

        let pipeline_id = match pipeline_bind_point {
            ash::vk::PipelineBindPoint::GRAPHICS => Accessor::from(pipeline).peek(),
            ash::vk::PipelineBindPoint::COMPUTE => {
                todo!()
            }
            _ => {
                panic!()
            }
        };

        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return;
            };

            let command = Command::BindRenderPipeline(pipeline_id);
            command_list.push(command);
        })
    }

    pub fn cmd_pipeline_barrier(
        _command_buffer: ash::vk::CommandBuffer,
        _src_stage_mask: ash::vk::PipelineStageFlags,
        _dst_stage_mask: ash::vk::PipelineStageFlags,
        _dependency_flags: ash::vk::DependencyFlags,
        _memory_barriers: &[ash::vk::MemoryBarrier<'_>],
        _buffer_memory_barriers: &[ash::vk::BufferMemoryBarrier<'_>],
        _image_memory_barriers: &[ash::vk::ImageMemoryBarrier<'_>],
    ) {
    }

    pub fn cmd_draw(
        command_buffer: ash::vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> ash::vk::Result {
        let (id, mut instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));

        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return;
            };

            let Some(command_list) = context.command_table.get_mut(&id) else {
                return;
            };

            let command = Command::Draw(DrawParameters {
                vertices: first_vertex..(first_vertex + vertex_count),
                instances: first_instance..(first_instance + instance_count),
            });
            command_list.push(command);
        });

        ash::vk::Result::SUCCESS
    }

    pub fn queue_submit(
        _queue: ash::vk::Queue,
        submits: &[ash::vk::SubmitInfo<'_>],
        _fence: ash::vk::Fence,
    ) -> ash::vk::Result {
        for submit_info in submits {
            let command_buffers: &[_] = unsafe {
                std::slice::from_raw_parts(
                    submit_info.p_command_buffers,
                    submit_info.command_buffer_count as usize,
                )
            };
            for command_buffer in command_buffers {
                let (id, instance_handle) = Accessor::from(*command_buffer)
                    .peek_return(|context: &_| (context.id, context.instance_handle));

                Accessor::from(instance_handle).peek(|context: &_| {
                    let device = &context.device;
                    let queue = &context.queue;
                    let Some(context) = &context.context else {
                        return;
                    };

                    let command_encoder = {
                        let command_encoder = device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
                        CommandEmulator::emulate(id, context, command_encoder)
                    };

                    queue.submit([command_encoder.finish()]);
                });
            }
        }

        ash::vk::Result::SUCCESS
    }

    pub fn queue_present(
        queue: ash::vk::Queue,
        _info: &ash::vk::PresentInfoKHR<'_>,
    ) -> ash::vk::Result {
        let mut instance_handle = Accessor::from(queue).peek().0;
        AccessorMut::from(&mut instance_handle).update(|context: &mut _| {
            let Some(context) = &mut context.context else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            let Some(surface_texture) = context.surface_texture.take() else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            surface_texture.present();
            ash::vk::Result::SUCCESS
        })
    }

    pub fn queue_wait_idle(queue: ash::vk::Queue) -> ash::vk::Result {
        let instance_handle = Accessor::from(queue).peek().0;
        Accessor::from(instance_handle).peek_return(|context: &_| {
            let Ok(_) = context.device.poll(wgpu::PollType::Wait) else {
                return ash::vk::Result::SUCCESS;
            };

            ash::vk::Result::SUCCESS
        })
    }

    pub fn device_wait_idle(device: ash::vk::Device) -> ash::vk::Result {
        let instance_handle = Accessor::from(device).peek().0;
        Accessor::from(instance_handle).peek_return(|context: &InstanceContext| {
            let Ok(_) = context.device.poll(wgpu::PollType::Wait) else {
                return ash::vk::Result::ERROR_UNKNOWN;
            };

            ash::vk::Result::SUCCESS
        })
    }
}

pub struct InstanceContext<'a> {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_table: HashMap<SurfaceId, wgpu::Surface<'a>>,
    surface_id_generator: IdGenerator<SurfaceId>,

    context: Option<Context>,
}

struct Context {
    command_pool_table: HashMap<CommandPoolId, HashSet<CommandBufferId>>,
    command_table: HashMap<CommandBufferId, Vec<Command>>,
    swapchain_image_table: HashMap<SwapchainId, Vec<ImageId>>,
    image_surface_table: HashMap<ImageId, SurfaceId>,

    image_view_table: HashMap<ImageViewId, ImageId>,
    pipeline_layout_table: HashMap<PipelineLayoutId, wgpu::PipelineLayout>,
    render_pipeline_table: HashMap<RenderPipelineId, wgpu::RenderPipeline>,
    #[allow(unused)]
    compute_pipeline_table: HashMap<ComputePipelineId, wgpu::ComputePipeline>,
    shader_module_table: HashMap<ShaderModuleId, wgpu::ShaderModule>,
    semaphore_table: HashSet<SemaphoreId>,

    image_id_generator: IdGenerator<ImageId>,
    image_view_id_generator: IdGenerator<ImageViewId>,
    command_pool_id_generator: IdGenerator<CommandPoolId>,
    command_buffer_id_generator: IdGenerator<CommandBufferId>,
    pipeline_layout_id_generator: IdGenerator<PipelineLayoutId>,
    render_pipeline_id_generator: IdGenerator<RenderPipelineId>,
    shader_module_id_generator: IdGenerator<ShaderModuleId>,
    semaphore_id_generator: IdGenerator<SemaphoreId>,
    swapchain_id_generator: IdGenerator<SwapchainId>,

    surface_texture: Option<wgpu::SurfaceTexture>,
}

struct CommandBufferContext {
    id: CommandBufferId,
    instance_handle: ash::vk::Instance,
}

vknew_macro::define_id!(CommandBufferId);
vknew_macro::define_id!(CommandPoolId);
vknew_macro::define_id!(ComputePipelineId);
vknew_macro::define_id!(DeviceId);
vknew_macro::define_id!(ImageId);
vknew_macro::define_id!(ImageViewId);
vknew_macro::define_id!(PipelineLayoutId);
vknew_macro::define_id!(RenderPipelineId);
vknew_macro::define_id!(QueueId);
vknew_macro::define_id!(SemaphoreId);
vknew_macro::define_id!(SurfaceId);
vknew_macro::define_id!(SwapchainId);
vknew_macro::define_id!(ShaderModuleId);

vknew_macro::generate_definition!(Accessor);

#[vknew_macro::generate_heap_impl(
    (ash::vk::Instance, InstanceContext<'static>),
    (ash::vk::CommandBuffer, CommandBufferContext),
)]
struct Accessor;

#[vknew_macro::generate_copyable_impl(
    (ash::vk::PhysicalDevice, InstanceHandleAdapter),
    (ash::vk::Queue, InstanceHandleAdapter),
    (ash::vk::CommandPool, CommandPoolId),
    (ash::vk::Device, InstanceHandleAdapter),
    (ash::vk::Image, ImageId),
    (ash::vk::ImageView, ImageViewId),
    (ash::vk::Semaphore, SemaphoreId),
    (ash::vk::ShaderModule, ShaderModuleId),
    (ash::vk::SurfaceKHR, SurfaceId),
    (ash::vk::Pipeline, RenderPipelineId),
    (ash::vk::PipelineLayout, PipelineLayoutId),
    (ash::vk::SwapchainKHR, SwapchainId)
)]
struct Accessor;

#[derive(Copy, Clone)]
struct InstanceHandleAdapter(ash::vk::Instance);

impl Into<u64> for InstanceHandleAdapter {
    fn into(self) -> u64 {
        self.0.as_raw()
    }
}

impl From<u64> for InstanceHandleAdapter {
    fn from(value: u64) -> Self {
        Self(ash::vk::Instance::from_raw(value))
    }
}

#[cfg(test)]
mod tests {
    use super::{CommandBufferId, QueueId};
    use crate::wgpu::details::compatibility_layer::IdGenerator;
    use ash::vk::Handle;
    use std::ffi::c_void;

    struct DeviceData {
        comment: String,
    }

    vknew_macro::generate_definition!(MyAccessor);

    #[vknew_macro::generate_heap_impl(
    (ash::vk::Device, DeviceData))]
    struct MyAccessor;

    #[vknew_macro::generate_copyable_impl(
    (ash::vk::Queue, QueueId),
    (ash::vk::CommandBuffer, CommandBufferId))]
    struct MyAccessor;

    #[test]
    fn copyable_works() {
        let id = {
            let mut id_generator = IdGenerator::new();
            id_generator.generate()
        };
        let mut handle = ash::vk::Queue::null();
        let dst_id = MyAccessorMut::from(&mut handle).with(id).peek();
        assert_eq!(id, dst_id);
    }

    #[test]
    fn heap_works() {
        let mut handle = ash::vk::Device::null();
        let _ = MyAccessorMut::from(&mut handle)
            .allocate(DeviceData {
                comment: "hello".to_string(),
            })
            .peek(|_| {});
        let _ =
            MyAccessor::from(handle).peek(|data: &DeviceData| assert_eq!(data.comment, "hello"));
        let _ = MyAccessorMut::from(&mut handle).free();
    }
}
