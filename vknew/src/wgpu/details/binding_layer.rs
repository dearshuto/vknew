use std::{
    ffi::{CStr, c_char, c_void},
    time::Duration,
};

use super::compatibility_layer::CompatibilityLayer;

// 関数バインディングレイヤー
pub struct BindingLayer;

impl BindingLayer {
    pub extern "system" fn get_instance_proc_addr(
        _instance: ash::vk::Instance,
        p_name: *const c_char,
    ) -> Option<unsafe extern "system" fn()> {
        let str = unsafe { CStr::from_ptr(p_name) }.to_str().unwrap();
        match str {
            "vkCreateInstance" => {
                Some(unsafe { std::mem::transmute(Self::create_instance as *mut c_void) })
            }
            "vkDestroyInstance" => {
                Some(unsafe { std::mem::transmute(Self::destroy_instance as *mut c_void) })
            }
            "vkEnumeratePhysicalDevices" => Some(unsafe {
                std::mem::transmute(Self::enumerate_physical_devices as *mut c_void)
            }),
            "vkGetDeviceProcAddr" => {
                Some(unsafe { std::mem::transmute(Self::get_device_proc_addr as *mut c_void) })
            }
            "vkCreateDevice" => {
                Some(unsafe { std::mem::transmute(Self::create_device as *mut c_void) })
            }
            "vkGetPhysicalDeviceSurfaceCapabilitiesKHR" => Some(unsafe {
                std::mem::transmute(
                    Self::get_physical_device_surface_capabilities_khr as *mut c_void,
                )
            }),
            "vkGetPhysicalDeviceSurfaceFormatsKHR" => Some(unsafe {
                std::mem::transmute(Self::get_physical_device_surface_formats_khr as *mut c_void)
            }),
            "vkDestroySurfaceKHR" => {
                Some(unsafe { std::mem::transmute(Self::destroy_surface as *mut c_void) })
            }
            _ => {
                // println!("instance proc: {}", str);
                None
            }
        }
    }

    fn get_device_proc_addr(
        _device: ash::vk::Device,
        p_name: *const c_char,
    ) -> Option<unsafe extern "system" fn()> {
        let Ok(str) = unsafe { CStr::from_ptr(p_name) }.to_str() else {
            return None;
        };

        match str {
            "vkDestroyDevice" => {
                Some(unsafe { std::mem::transmute(Self::destroy_device as *mut c_void) })
            }
            "vkGetDeviceQueue" => {
                Some(unsafe { std::mem::transmute(Self::get_device_queue as *mut c_void) })
            }
            "vkCreateCommandPool" => {
                Some(unsafe { std::mem::transmute(Self::create_command_pool as *mut c_void) })
            }
            "vkDestroyCommandPool" => {
                Some(unsafe { std::mem::transmute(Self::destroy_command_pool as *mut c_void) })
            }
            "vkAllocateCommandBuffers" => {
                Some(unsafe { std::mem::transmute(Self::allocate_command_buffers as *mut c_void) })
            }
            "vkFreeCommandBuffers" => {
                Some(unsafe { std::mem::transmute(Self::free_command_buffers as *mut c_void) })
            }
            "vkQueueSubmit" => {
                Some(unsafe { std::mem::transmute(Self::queue_submit as *mut c_void) })
            }
            "vkQueueWaitIdle" => {
                Some(unsafe { std::mem::transmute(Self::queue_wait_idle as *mut c_void) })
            }
            "vkCreateShaderModule" => {
                Some(unsafe { std::mem::transmute(Self::create_shader_module as *mut c_void) })
            }
            "vkDestroyShaderModule" => {
                Some(unsafe { std::mem::transmute(Self::destroy_shader_module as *mut c_void) })
            }
            "vkCreateSwapchainKHR" => {
                Some(unsafe { std::mem::transmute(Self::create_swapchain_khr as *mut c_void) })
            }
            "vkCreatePipelineLayout" => {
                Some(unsafe { std::mem::transmute(Self::create_pipeline_layout as *mut c_void) })
            }
            "vkDestroyPipelineLayout" => {
                Some(unsafe { std::mem::transmute(Self::destroy_pipeline_layout as *mut c_void) })
            }
            "vkCreateGraphicsPipelines" => {
                Some(unsafe { std::mem::transmute(Self::create_graphics_pipelines as *mut c_void) })
            }
            "vkDestroyPipeline" => {
                Some(unsafe { std::mem::transmute(Self::destroy_pipeline as *mut c_void) })
            }
            "vkCreateImageView" => {
                Some(unsafe { std::mem::transmute(Self::create_image_view as *mut c_void) })
            }
            "vkDestroyImageView" => {
                Some(unsafe { std::mem::transmute(Self::destroy_image_view as *mut c_void) })
            }
            "vkDestroySwapchainKHR" => {
                Some(unsafe { std::mem::transmute(Self::destroy_swapchain_khr as *mut c_void) })
            }
            "vkGetSwapchainImagesKHR" => {
                Some(unsafe { std::mem::transmute(Self::get_swapchain_images_khr as *mut c_void) })
            }
            "vkAcquireNextImageKHR" => {
                Some(unsafe { std::mem::transmute(Self::acquire_next_image_khr as *mut c_void) })
            }
            "vkCreateSemaphore" => {
                Some(unsafe { std::mem::transmute(Self::create_semaphore as *mut c_void) })
            }
            "vkDestroySemaphore" => {
                Some(unsafe { std::mem::transmute(Self::destroy_semaphore as *mut c_void) })
            }
            "vkResetCommandBuffer" => {
                Some(unsafe { std::mem::transmute(Self::reset_command_buffer as *mut c_void) })
            }
            "vkBeginCommandBuffer" => {
                Some(unsafe { std::mem::transmute(Self::begin_command_buffer as *mut c_void) })
            }
            "vkEndCommandBuffer" => {
                Some(unsafe { std::mem::transmute(Self::end_command_buffer as *mut c_void) })
            }
            "vkCmdBeginRendering" => {
                Some(unsafe { std::mem::transmute(Self::cmd_begin_rendering as *mut c_void) })
            }
            "vkCmdEndRendering" => {
                Some(unsafe { std::mem::transmute(Self::cmd_end_rendering as *mut c_void) })
            }
            "vkCmdBeginRenderingKHR" => {
                Some(unsafe { std::mem::transmute(Self::cmd_begin_rendering as *mut c_void) })
            }
            "vkCmdEndRenderingKHR" => {
                Some(unsafe { std::mem::transmute(Self::cmd_end_rendering as *mut c_void) })
            }
            "vkCmdBindPipeline" => {
                Some(unsafe { std::mem::transmute(Self::cmd_bind_pipeline as *mut c_void) })
            }
            "vkCmdPipelineBarrier" => {
                Some(unsafe { std::mem::transmute(Self::cmd_pipeline_barrier as *mut c_void) })
            }
            "vkCmdDraw" => Some(unsafe { std::mem::transmute(Self::cmd_draw as *mut c_void) }),
            "vkQueuePresentKHR" => {
                Some(unsafe { std::mem::transmute(Self::queue_present as *mut c_void) })
            }
            "vkDeviceWaitIdle" => {
                Some(unsafe { std::mem::transmute(Self::device_wait_idle as *mut c_void) })
            }
            _ => {
                // println!("device proc: {}", str);
                None
            }
        }
    }
}

impl BindingLayer {
    fn create_instance(
        p_create_info: *const ash::vk::InstanceCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_instance: *mut ash::vk::Instance,
    ) -> i32 {
        if p_create_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let create_info = unsafe { &*p_create_info };
        let dst_instance = unsafe { &mut *p_instance };
        CompatibilityLayer::create_instance(create_info, to_opt(p_allocator), dst_instance).as_raw()
    }

    fn destroy_instance(
        instance: ash::vk::Instance,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_instance(instance, to_opt(p_allocator));
    }

    fn enumerate_physical_devices(
        instance: ash::vk::Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut ash::vk::PhysicalDevice,
    ) -> i32 {
        // デバイスにヌルが指定されたら要素数を返す
        if p_physical_devices == std::ptr::null_mut() {
            unsafe { *p_physical_device_count = 1 };
            return ash::vk::Result::SUCCESS.as_raw();
        }

        // デバイスは少なくとも 1 つは存在するとする
        // MEMO: 実際は adapter から問い合わせた結果で判定する
        let count = unsafe { *p_physical_device_count } as usize;
        let dst_physical_devices =
            unsafe { std::slice::from_raw_parts_mut(p_physical_devices, count) };
        CompatibilityLayer::enumerate_physical_devices(instance, dst_physical_devices).as_raw()
    }

    fn get_physical_device_surface_capabilities_khr(
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
        p_surface_capabilities: *mut ash::vk::SurfaceCapabilitiesKHR,
    ) -> i32 {
        let dst_surface_capabilities = unsafe { &mut *p_surface_capabilities };
        CompatibilityLayer::get_physical_device_surface_capabilities_khr(
            physical_device,
            surface,
            dst_surface_capabilities,
        )
        .as_raw()
    }

    fn get_physical_device_surface_formats_khr(
        physical_device: ash::vk::PhysicalDevice,
        surface: ash::vk::SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut ash::vk::SurfaceFormatKHR,
    ) -> i32 {
        if p_surface_format_count == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        if p_surface_formats == std::ptr::null_mut() {
            unsafe { *p_surface_format_count = 1 };
            return ash::vk::Result::SUCCESS.as_raw();
        }

        let dst_formats = unsafe {
            std::slice::from_raw_parts_mut(p_surface_formats, *p_surface_format_count as usize)
        };
        if dst_formats.is_empty() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        CompatibilityLayer::get_physical_device_surface_formats_khr(
            physical_device,
            surface,
            dst_formats,
        )
        .as_raw()
    }

    fn create_device(
        physical_device: ash::vk::PhysicalDevice,
        p_create_info: *const ash::vk::DeviceCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_device: *mut ash::vk::Device,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_device == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }
        let create_info = unsafe { &*p_create_info };
        let device = unsafe { &mut *p_device };

        CompatibilityLayer::create_device(
            physical_device,
            &create_info,
            to_opt(p_allocator),
            device,
        )
        .as_raw()
    }

    fn destroy_device(
        device: ash::vk::Device,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_device(device, to_opt(p_allocator));
    }

    fn get_device_queue(
        device: ash::vk::Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut ash::vk::Queue,
    ) {
        if p_queue == std::ptr::null_mut() {
            return;
        }

        let queue = unsafe { &mut *p_queue };
        CompatibilityLayer::get_device_queue(device, queue_family_index, queue_index, queue);
    }

    fn destroy_surface(
        instance: ash::vk::Instance,
        surface: ash::vk::SurfaceKHR,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_surface(instance, surface, to_opt(p_allocator));
    }

    fn create_swapchain_khr(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::SwapchainCreateInfoKHR<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_swapchain: *mut ash::vk::SwapchainKHR,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_swapchain == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let create_info = unsafe { &*p_create_info };
        let swapchain = unsafe { &mut *p_swapchain };
        CompatibilityLayer::create_swapchain_khr(
            device,
            create_info,
            to_opt(p_allocator),
            swapchain,
        )
        .as_raw()
    }

    fn destroy_swapchain_khr(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_swapchain_khr(device, swapchain, to_opt(p_allocator))
    }

    fn get_swapchain_images_khr(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut ash::vk::Image,
    ) -> i32 {
        if p_swapchain_image_count == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        // 要素数を取得
        if p_swapchain_images == std::ptr::null_mut() {
            return CompatibilityLayer::get_swapchain_image_count(device, swapchain, unsafe {
                &mut *p_swapchain_image_count
            })
            .as_raw();
        }

        // 出力バッファーに書き込む
        let dst_images = unsafe {
            std::slice::from_raw_parts_mut(p_swapchain_images, *p_swapchain_image_count as usize)
        };
        CompatibilityLayer::get_swapchain_images_khr(device, swapchain, dst_images).as_raw()
    }

    fn acquire_next_image_khr(
        device: ash::vk::Device,
        swapchain: ash::vk::SwapchainKHR,
        timeout: u64,
        semaphore: ash::vk::Semaphore,
        fence: ash::vk::Fence,
        p_image_index: *mut u32,
    ) -> i32 {
        let timeout = Duration::from_nanos(timeout);
        CompatibilityLayer::acquire_next_image_khr(
            device,
            swapchain,
            timeout,
            semaphore,
            fence,
            unsafe { &mut *p_image_index },
        )
        .as_raw()
    }

    fn create_command_pool(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::CommandPoolCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_command_pool: *mut ash::vk::CommandPool,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_command_pool == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let create_info = unsafe { &*p_create_info };
        let allocator = to_opt(p_allocator);
        let command_pool = unsafe { &mut *p_command_pool };
        CompatibilityLayer::create_command_pool(device, create_info, allocator, command_pool)
            .as_raw()
    }

    fn destroy_command_pool(
        device: ash::vk::Device,
        command_pool: ash::vk::CommandPool,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) -> i32 {
        CompatibilityLayer::destroy_command_pool(device, command_pool, to_opt(p_allocator)).as_raw()
    }

    fn allocate_command_buffers(
        device: ash::vk::Device,
        p_allocate_info: *const ash::vk::CommandBufferAllocateInfo<'_>,
        p_command_buffers: *mut ash::vk::CommandBuffer,
    ) -> i32 {
        if p_allocate_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let allocate_info = unsafe { &*p_allocate_info };
        let dst_command_buffers = unsafe {
            std::slice::from_raw_parts_mut(
                p_command_buffers,
                allocate_info.command_buffer_count as usize,
            )
        };

        CompatibilityLayer::allocate_command_buffers(device, allocate_info, dst_command_buffers)
            .as_raw()
    }

    fn free_command_buffers(
        device: ash::vk::Device,
        command_pool: ash::vk::CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const ash::vk::CommandBuffer,
    ) -> i32 {
        let command_buffers =
            unsafe { std::slice::from_raw_parts(p_command_buffers, command_buffer_count as usize) };
        CompatibilityLayer::free_command_buffers(device, command_pool, command_buffers).as_raw()
    }

    fn create_shader_module(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::ShaderModuleCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_shader_module: *mut ash::vk::ShaderModule,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_shader_module == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        unsafe {
            CompatibilityLayer::create_shader_module(
                device,
                &*p_create_info,
                to_opt(p_allocator),
                &mut *p_shader_module,
            )
            .as_raw()
        }
    }

    fn destroy_shader_module(
        device: ash::vk::Device,
        shader_module: ash::vk::ShaderModule,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_shader_module(device, shader_module, to_opt(p_allocator))
    }

    fn create_pipeline_layout(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::PipelineLayoutCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_pipeline_layout: *mut ash::vk::PipelineLayout,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_pipeline_layout == std::ptr::null_mut() {
            return ash::vk::Result::SUCCESS.as_raw();
        }

        let create_info = unsafe { &*p_create_info };
        let pipeline_layout = unsafe { &mut *p_pipeline_layout };
        CompatibilityLayer::create_pipeline_layout(
            device,
            create_info,
            to_opt(p_allocator),
            pipeline_layout,
        )
        .as_raw()
    }

    fn destroy_pipeline_layout(
        device: ash::vk::Device,
        pipeline_layout: ash::vk::PipelineLayout,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_pipeline_layout(device, pipeline_layout, to_opt(p_allocator));
    }

    fn create_graphics_pipelines(
        device: ash::vk::Device,
        pipeline_cache: ash::vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ash::vk::GraphicsPipelineCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_pipelines: *mut ash::vk::Pipeline,
    ) -> i32 {
        if p_pipelines == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let infos =
            unsafe { std::slice::from_raw_parts(p_create_infos, create_info_count as usize) };
        let pipelines =
            unsafe { std::slice::from_raw_parts_mut(p_pipelines, create_info_count as usize) };

        CompatibilityLayer::create_graphics_pipelines(
            device,
            pipeline_cache,
            infos,
            to_opt(p_allocator),
            pipelines,
        )
        .as_raw()
    }

    fn destroy_pipeline(
        device: ash::vk::Device,
        pipeline: ash::vk::Pipeline,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_pipeline(device, pipeline, to_opt(p_allocator))
    }

    fn create_image_view(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::ImageViewCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_view: *mut ash::vk::ImageView,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_view == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let create_info = unsafe { &*p_create_info };
        let dst_view = unsafe { &mut *p_view };
        CompatibilityLayer::create_image_view(device, create_info, to_opt(p_allocator), dst_view)
            .as_raw()
    }

    fn destroy_image_view(
        device: ash::vk::Device,
        image_view: ash::vk::ImageView,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_image_view(device, image_view, to_opt(p_allocator));
    }

    fn create_semaphore(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::SemaphoreCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_semaphore: *mut ash::vk::Semaphore,
    ) -> i32 {
        if p_create_info == std::ptr::null() || p_semaphore == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let create_info = unsafe { &*p_create_info };
        let dst_semaphore = unsafe { &mut *p_semaphore };
        CompatibilityLayer::create_semaphore(
            device,
            create_info,
            to_opt(p_allocator),
            dst_semaphore,
        )
        .as_raw()
    }

    fn destroy_semaphore(
        device: ash::vk::Device,
        semaphore: ash::vk::Semaphore,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_semaphore(device, semaphore, to_opt(p_allocator));
    }

    fn reset_command_buffer(
        command_buffer: ash::vk::CommandBuffer,
        flags: ash::vk::CommandBufferResetFlags,
    ) -> i32 {
        CompatibilityLayer::reset_command_buffer(command_buffer, flags).as_raw()
    }

    fn begin_command_buffer(
        command_buffer: ash::vk::CommandBuffer,
        p_begin_info: *const ash::vk::CommandBufferBeginInfo<'_>,
    ) -> i32 {
        if p_begin_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let begin_info = unsafe { &*p_begin_info };
        CompatibilityLayer::begin_command_buffer(command_buffer, begin_info).as_raw()
    }

    fn end_command_buffer(
        command_buffer: ash::vk::CommandBuffer,
        flags: ash::vk::CommandBufferResetFlags,
    ) -> i32 {
        CompatibilityLayer::end_command_buffer(command_buffer, flags).as_raw()
    }

    fn cmd_begin_rendering(
        command_buffer: ash::vk::CommandBuffer,
        p_rendering_info: *const ash::vk::RenderingInfo<'_>,
    ) {
        if p_rendering_info == std::ptr::null() {
            return;
        }

        let info = unsafe { &*p_rendering_info };
        CompatibilityLayer::cmd_begin_rendering(command_buffer, info);
    }

    fn cmd_end_rendering(command_buffer: ash::vk::CommandBuffer) {
        CompatibilityLayer::cmd_end_rendering(command_buffer);
    }

    fn cmd_bind_pipeline(
        command_buffer: ash::vk::CommandBuffer,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        pipeline: ash::vk::Pipeline,
    ) {
        CompatibilityLayer::cmd_bind_pipeline(command_buffer, pipeline_bind_point, pipeline);
    }

    fn cmd_pipeline_barrier(
        command_buffer: ash::vk::CommandBuffer,
        src_stage_mask: ash::vk::PipelineStageFlags,
        dst_stage_mask: ash::vk::PipelineStageFlags,
        dependency_flags: ash::vk::DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const ash::vk::MemoryBarrier<'_>,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const ash::vk::BufferMemoryBarrier<'_>,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ash::vk::ImageMemoryBarrier<'_>,
    ) {
        let memory_barriers =
            unsafe { std::slice::from_raw_parts(p_memory_barriers, memory_barrier_count as usize) };
        let buffer_memory_barriers = unsafe {
            std::slice::from_raw_parts(
                p_buffer_memory_barriers,
                buffer_memory_barrier_count as usize,
            )
        };
        let image_memory_barriers = unsafe {
            std::slice::from_raw_parts(p_image_memory_barriers, image_memory_barrier_count as usize)
        };

        CompatibilityLayer::cmd_pipeline_barrier(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barriers,
            buffer_memory_barriers,
            image_memory_barriers,
        );
    }

    fn cmd_draw(
        command_buffer: ash::vk::CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> i32 {
        CompatibilityLayer::cmd_draw(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
        .as_raw()
    }

    fn queue_submit(
        queue: ash::vk::Queue,
        submit_count: u32,
        p_submits: *const ash::vk::SubmitInfo<'_>,
        fence: ash::vk::Fence,
    ) -> i32 {
        let submits = unsafe { std::slice::from_raw_parts(p_submits, submit_count as usize) };
        CompatibilityLayer::queue_submit(queue, submits, fence).as_raw()
    }

    fn queue_wait_idle(queue: ash::vk::Queue) -> i32 {
        CompatibilityLayer::queue_wait_idle(queue).as_raw()
    }

    fn queue_present(
        queue: ash::vk::Queue,
        p_present_info: *const ash::vk::PresentInfoKHR<'_>,
    ) -> i32 {
        if p_present_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let info = unsafe { &*p_present_info };
        CompatibilityLayer::queue_present(queue, info).as_raw()
    }

    fn device_wait_idle(device: ash::vk::Device) -> i32 {
        CompatibilityLayer::device_wait_idle(device).as_raw()
    }
}

fn to_opt(
    p_allocator: *const ash::vk::AllocationCallbacks<'_>,
) -> Option<&ash::vk::AllocationCallbacks<'_>> {
    if p_allocator != std::ptr::null() {
        Some(unsafe { &*p_allocator })
    } else {
        None
    }
}
