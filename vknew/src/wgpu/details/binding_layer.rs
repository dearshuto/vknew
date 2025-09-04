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
            "vkGetPhysicalDeviceMemoryProperties" => Some(unsafe {
                std::mem::transmute(Self::get_physical_device_memory_properties as *mut c_void)
            }),
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
            "vkGetBufferMemoryRequirements" => Some(unsafe {
                std::mem::transmute(Self::get_buffer_memory_requirements as *mut c_void)
            }),
            "vkAllocateMemory" => {
                Some(unsafe { std::mem::transmute(Self::allocate_memory as *mut c_void) })
            }
            "vkFreeMemory" => {
                Some(unsafe { std::mem::transmute(Self::free_memory as *mut c_void) })
            }
            "vkMapMemory" => Some(unsafe { std::mem::transmute(Self::map_memory as *mut c_void) }),
            "vkUnmapMemory" => {
                Some(unsafe { std::mem::transmute(Self::unmap_memory as *mut c_void) })
            }
            "vkBindBufferMemory" => {
                Some(unsafe { std::mem::transmute(Self::bind_buffer_memory as *mut c_void) })
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
            "vkCreateDescriptorPool" => {
                Some(unsafe { std::mem::transmute(Self::create_descriptor_pool as *mut c_void) })
            }
            "vkDestroyDescriptorPool" => {
                Some(unsafe { std::mem::transmute(Self::destroy_descriptor_pool as *mut c_void) })
            }
            "vkAllocateDescriptorSets" => {
                Some(unsafe { std::mem::transmute(Self::allocate_descriptor_sets as *mut c_void) })
            }
            "vkCreateDescriptorSetLayout" => Some(unsafe {
                std::mem::transmute(Self::create_descriptor_set_layout as *mut c_void)
            }),
            "vkDestroyDescriptorSetLayout" => Some(unsafe {
                std::mem::transmute(Self::destroy_descriptor_set_layout as *mut c_void)
            }),
            "vkUpdateDescriptorSets" => {
                Some(unsafe { std::mem::transmute(Self::update_descriptor_sets as *mut c_void) })
            }
            "vkCreateGraphicsPipelines" => {
                Some(unsafe { std::mem::transmute(Self::create_graphics_pipelines as *mut c_void) })
            }
            "vkCreateComputePipelines" => {
                Some(unsafe { std::mem::transmute(Self::create_compute_pipelines as *mut c_void) })
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
            "vkCreateBuffer" => {
                Some(unsafe { std::mem::transmute(Self::create_buffer as *mut c_void) })
            }
            "vkDestroyBuffer" => {
                Some(unsafe { std::mem::transmute(Self::destroy_buffer as *mut c_void) })
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
            "vkCmdBindDescriptorSets" => {
                Some(unsafe { std::mem::transmute(Self::cmd_bind_descriptor_sets as *mut c_void) })
            }
            "vkCmdBindVertexBuffers" => {
                Some(unsafe { std::mem::transmute(Self::cmd_bind_vertex_buffers as *mut c_void) })
            }
            "vkCmdPipelineBarrier" => {
                Some(unsafe { std::mem::transmute(Self::cmd_pipeline_barrier as *mut c_void) })
            }
            "vkCmdDraw" => Some(unsafe { std::mem::transmute(Self::cmd_draw as *mut c_void) }),
            "vkCmdDispatch" => {
                Some(unsafe { std::mem::transmute(Self::cmd_dispatch as *mut c_void) })
            }
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

    fn get_physical_device_memory_properties(
        physical_device: ash::vk::PhysicalDevice,
        p_memory_properties: *mut ash::vk::PhysicalDeviceMemoryProperties,
    ) {
        if p_memory_properties == std::ptr::null_mut() {
            return;
        }

        let dst_memory_properties = unsafe { &mut *p_memory_properties };
        CompatibilityLayer::get_physical_device_memory_properties(
            physical_device,
            dst_memory_properties,
        );
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

    fn get_buffer_memory_requirements(
        device: ash::vk::Device,
        buffer: ash::vk::Buffer,
        p_memory_requirements: *mut ash::vk::MemoryRequirements,
    ) {
        if p_memory_requirements == std::ptr::null_mut() {
            return;
        }

        let dst_memory_requirements = unsafe { &mut *p_memory_requirements };
        CompatibilityLayer::get_buffer_memory_requirements(device, buffer, dst_memory_requirements);
    }

    fn allocate_memory(
        device: ash::vk::Device,
        p_allocate_info: *const ash::vk::MemoryAllocateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_memory: *mut ash::vk::DeviceMemory,
    ) -> i32 {
        if p_allocate_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        if p_memory == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let allocate_info = unsafe { &*p_allocate_info };
        let dst_memory = unsafe { &mut *p_memory };
        CompatibilityLayer::allocate_memory(device, allocate_info, to_opt(p_allocator), dst_memory)
            .as_raw()
    }

    fn free_memory(
        device: ash::vk::Device,
        memory: ash::vk::DeviceMemory,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        if p_allocator == std::ptr::null_mut() {
            return;
        }

        CompatibilityLayer::free_memory(device, memory, to_opt(p_allocator));
    }

    fn map_memory(
        device: ash::vk::Device,
        memory: ash::vk::DeviceMemory,
        offset: ash::vk::DeviceSize,
        size: ash::vk::DeviceSize,
        flags: ash::vk::MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> i32 {
        if pp_data == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        CompatibilityLayer::map_memory(device, memory, offset, size, flags, pp_data).as_raw()
    }

    fn unmap_memory(device: ash::vk::Device, memory: ash::vk::DeviceMemory) {
        CompatibilityLayer::unmap_memory(device, memory);
    }

    fn bind_buffer_memory(
        device: ash::vk::Device,
        buffer: ash::vk::Buffer,
        memory: ash::vk::DeviceMemory,
        memory_offset: ash::vk::DeviceSize,
    ) -> i32 {
        CompatibilityLayer::bind_buffer_memory(device, buffer, memory, memory_offset).as_raw()
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

    fn create_descriptor_pool(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::DescriptorPoolCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_descriptor_pool: *mut ash::vk::DescriptorPool,
    ) -> i32 {
        let info = unsafe { &*p_create_info };
        let dst_descriptor_pool = unsafe { &mut *p_descriptor_pool };
        CompatibilityLayer::create_descriptor_pool(
            device,
            info,
            to_opt(p_allocator),
            dst_descriptor_pool,
        )
        .as_raw()
    }

    fn destroy_descriptor_pool(
        device: ash::vk::Device,
        descriptor_pool: ash::vk::DescriptorPool,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_descriptor_pool(device, descriptor_pool, to_opt(p_allocator));
    }

    fn allocate_descriptor_sets(
        device: ash::vk::Device,
        p_allocate_info: *const ash::vk::DescriptorSetAllocateInfo<'_>,
        p_descriptor_sets: *mut ash::vk::DescriptorSet,
    ) -> i32 {
        if p_allocate_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let info = unsafe { &*p_allocate_info };
        let dst_descriptor_sets = unsafe {
            std::slice::from_raw_parts_mut(p_descriptor_sets, info.descriptor_set_count as usize)
        };

        CompatibilityLayer::allocate_descriptor_sets(device, info, dst_descriptor_sets).as_raw()
    }

    fn create_descriptor_set_layout(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::DescriptorSetLayoutCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_set_layout: *mut ash::vk::DescriptorSetLayout,
    ) -> i32 {
        let info = unsafe { &*p_create_info };
        let dst_layout = unsafe { &mut *p_set_layout };
        CompatibilityLayer::create_descriptor_set_layout(
            device,
            info,
            to_opt(p_allocator),
            dst_layout,
        )
        .as_raw()
    }

    fn destroy_descriptor_set_layout(
        device: ash::vk::Device,
        descriptor_set_layout: ash::vk::DescriptorSetLayout,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_descriptor_set_layout(
            device,
            descriptor_set_layout,
            to_opt(p_allocator),
        );
    }

    fn update_descriptor_sets(
        device: ash::vk::Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const ash::vk::WriteDescriptorSet<'_>,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const ash::vk::CopyDescriptorSet<'_>,
    ) {
        let descriptor_writes = unsafe {
            std::slice::from_raw_parts(p_descriptor_writes, descriptor_write_count as usize)
        };
        let descriptor_copies = unsafe {
            std::slice::from_raw_parts(p_descriptor_copies, descriptor_copy_count as usize)
        };
        CompatibilityLayer::update_descriptor_sets(device, descriptor_writes, descriptor_copies);
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

    fn create_compute_pipelines(
        device: ash::vk::Device,
        pipeline_cache: ash::vk::PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ash::vk::ComputePipelineCreateInfo<'_>,
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

        CompatibilityLayer::create_compute_pipelines(
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

    fn create_buffer(
        device: ash::vk::Device,
        p_create_info: *const ash::vk::BufferCreateInfo<'_>,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
        p_buffer: *mut ash::vk::Buffer,
    ) -> i32 {
        if p_create_info == std::ptr::null() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        if p_buffer == std::ptr::null_mut() {
            return ash::vk::Result::ERROR_UNKNOWN.as_raw();
        }

        let info = unsafe { &*p_create_info };
        let dst_buffer = unsafe { &mut *p_buffer };
        CompatibilityLayer::create_buffer(device, info, to_opt(p_allocator), dst_buffer).as_raw()
    }

    fn destroy_buffer(
        device: ash::vk::Device,
        buffer: ash::vk::Buffer,
        p_allocator: *const ash::vk::AllocationCallbacks<'_>,
    ) {
        CompatibilityLayer::destroy_buffer(device, buffer, to_opt(p_allocator));
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

    fn cmd_bind_descriptor_sets(
        command_buffer: ash::vk::CommandBuffer,
        pipeline_bind_point: ash::vk::PipelineBindPoint,
        layout: ash::vk::PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const ash::vk::DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        let descriptor_sets =
            unsafe { std::slice::from_raw_parts(p_descriptor_sets, descriptor_set_count as usize) };
        let dynamic_offsets =
            unsafe { std::slice::from_raw_parts(p_dynamic_offsets, dynamic_offset_count as usize) };
        CompatibilityLayer::cmd_bind_descriptor_sets(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_sets,
            dynamic_offsets,
        );
    }

    fn cmd_bind_vertex_buffers(
        command_buffer: ash::vk::CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const ash::vk::Buffer,
        p_offsets: *const ash::vk::DeviceSize,
    ) {
        if p_buffers == std::ptr::null() {
            return;
        }

        if p_offsets == std::ptr::null() {
            return;
        }

        let buffers = unsafe { std::slice::from_raw_parts(p_buffers, binding_count as usize) };
        let offsets = unsafe { std::slice::from_raw_parts(p_offsets, binding_count as usize) };
        CompatibilityLayer::cmd_bind_vertex_buffers(
            command_buffer,
            first_binding,
            buffers,
            offsets,
        );
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

    fn cmd_dispatch(
        command_buffer: ash::vk::CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        CompatibilityLayer::cmd_dispatch(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        );
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
