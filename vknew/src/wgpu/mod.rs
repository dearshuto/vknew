mod details;
mod extensions;
mod surface;

use std::ffi::c_void;

use ash::vk::TaggedStructure;

use crate::wgpu::details::BindingLayer;

pub fn get_static_fn() -> ash::StaticFn {
    ash::StaticFn {
        get_instance_proc_addr: BindingLayer::get_instance_proc_addr,
    }
}

pub use surface::create_surface;

#[repr(C)]
pub struct ExtensionCreateInfoBase {
    pub s_type: ash::vk::StructureType,
    pub p_next: *const c_void,
}

#[repr(align(8))]
#[repr(C)]
pub struct WasmCompatibilityCreateInfo {
    pub s_type: ash::vk::StructureType,
    pub p_next: *const c_void,
    pub instance: Option<wgpu::Instance>,
    pub device: Option<wgpu::Device>,
    pub queue: Option<wgpu::Queue>,
    pub adapter: Option<wgpu::Adapter>,
}

impl WasmCompatibilityCreateInfo {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::default();
        let Ok(adapter) = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
        else {
            todo!();
        };

        let Ok((device, queue)) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
                trace: wgpu::Trace::Off,
            })
            .await
        else {
            todo!()
        };

        Self {
            s_type: WasmCompatibilityCreateInfo::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            instance: Some(instance),
            device: Some(device),
            queue: Some(queue),
            adapter: Some(adapter),
        }
    }

    pub fn from_instance(device: wgpu::Device, queue: wgpu::Queue, adapter: wgpu::Adapter) -> Self {
        // 既存のインスタンスから作成した場合は wgpu::Instnace だけないものとする
        // これは vknew 内でサーフェイスが作れないことを意味するが、この API はサーフェイスの管理は外部に委託するモードということにする
        Self {
            s_type: WasmCompatibilityCreateInfo::STRUCTURE_TYPE,
            p_next: std::ptr::null(),
            instance: None,
            device: Some(device),
            queue: Some(queue),
            adapter: Some(adapter),
        }
    }
}

unsafe impl ash::vk::ExtendsInstanceCreateInfo for WasmCompatibilityCreateInfo {}
unsafe impl ash::vk::TaggedStructure for WasmCompatibilityCreateInfo {
    const STRUCTURE_TYPE: ash::vk::StructureType = ash::vk::StructureType::from_raw(100);
}
