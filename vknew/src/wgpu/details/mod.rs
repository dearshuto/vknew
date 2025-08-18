mod binding_layer;
mod compatibility_layer;

pub use binding_layer::BindingLayer;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

pub fn create_surface_impl(
    entry: &ash::Entry,
    instance: &ash::Instance,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
    allocation_callbacks: Option<&ash::vk::AllocationCallbacks<'_>>,
) -> Result<ash::vk::SurfaceKHR, ash::vk::Result> {
    compatibility_layer::CompatibilityLayer::create_surface(
        entry,
        instance,
        display_handle,
        window_handle,
        allocation_callbacks,
    )
}
