use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::wgpu::details::create_surface_impl;

pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
    allocation_callbacks: Option<&ash::vk::AllocationCallbacks<'_>>,
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
    create_surface_impl(
        entry,
        instance,
        display_handle,
        window_handle,
        allocation_callbacks,
    )
}
