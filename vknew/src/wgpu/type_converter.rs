pub struct TextureFormat;
impl TextureFormat {
    pub fn to_vk(format: wgpu::TextureFormat) -> ash::vk::Format {
        match format {
            wgpu::TextureFormat::R8Unorm => ash::vk::Format::R8_UNORM,
            wgpu::TextureFormat::R8Snorm => ash::vk::Format::R8_SNORM,
            wgpu::TextureFormat::R8Uint => ash::vk::Format::R8_UINT,
            wgpu::TextureFormat::R8Sint => ash::vk::Format::R8_SINT,
            wgpu::TextureFormat::R16Uint => ash::vk::Format::R16_UINT,
            wgpu::TextureFormat::R16Sint => ash::vk::Format::R16_SINT,
            wgpu::TextureFormat::R16Unorm => ash::vk::Format::R16_UNORM,
            wgpu::TextureFormat::R16Snorm => ash::vk::Format::R16_SNORM,
            wgpu::TextureFormat::R16Float => ash::vk::Format::R16_SFLOAT,
            wgpu::TextureFormat::Rg8Unorm => ash::vk::Format::R8G8_UNORM,
            wgpu::TextureFormat::Rg8Snorm => ash::vk::Format::R8G8_SNORM,
            wgpu::TextureFormat::Rg8Uint => ash::vk::Format::R8G8_UINT,
            wgpu::TextureFormat::Rg8Sint => ash::vk::Format::R8G8_SINT,
            wgpu::TextureFormat::R32Uint => ash::vk::Format::R32_UINT,
            wgpu::TextureFormat::R32Sint => ash::vk::Format::R32_SINT,
            wgpu::TextureFormat::R32Float => ash::vk::Format::R32_SFLOAT,
            wgpu::TextureFormat::Rg16Uint => ash::vk::Format::R16G16_UINT,
            wgpu::TextureFormat::Rg16Sint => ash::vk::Format::R16G16_SINT,
            wgpu::TextureFormat::Rg16Unorm => ash::vk::Format::R16G16_UNORM,
            wgpu::TextureFormat::Rg16Snorm => ash::vk::Format::R16G16_SNORM,
            wgpu::TextureFormat::Rg16Float => ash::vk::Format::R16G16_SNORM,
            wgpu::TextureFormat::Rgba8Unorm => ash::vk::Format::R8G8B8A8_UNORM,
            wgpu::TextureFormat::Rgba8UnormSrgb => ash::vk::Format::R8G8B8A8_SRGB,
            wgpu::TextureFormat::Rgba8Snorm => ash::vk::Format::R8G8B8A8_SNORM,
            wgpu::TextureFormat::Rgba8Uint => ash::vk::Format::R8G8B8A8_UINT,
            wgpu::TextureFormat::Rgba8Sint => ash::vk::Format::R8G8B8A8_SINT,
            wgpu::TextureFormat::Bgra8Unorm => ash::vk::Format::B8G8R8A8_UNORM,
            wgpu::TextureFormat::Bgra8UnormSrgb => ash::vk::Format::B8G8R8A8_SRGB,
            wgpu::TextureFormat::Rgb9e5Ufloat => todo!(),
            wgpu::TextureFormat::Rgb10a2Uint => todo!(),
            wgpu::TextureFormat::Rgb10a2Unorm => todo!(),
            wgpu::TextureFormat::Rg11b10Ufloat => todo!(),
            wgpu::TextureFormat::R64Uint => ash::vk::Format::R64_UINT,
            wgpu::TextureFormat::Rg32Uint => ash::vk::Format::R32G32_UINT,
            wgpu::TextureFormat::Rg32Sint => ash::vk::Format::R32G32_SINT,
            wgpu::TextureFormat::Rg32Float => ash::vk::Format::R32G32_SFLOAT,
            wgpu::TextureFormat::Rgba16Uint => ash::vk::Format::R16G16B16A16_UINT,
            wgpu::TextureFormat::Rgba16Sint => ash::vk::Format::R16G16B16A16_SINT,
            wgpu::TextureFormat::Rgba16Unorm => ash::vk::Format::R16G16B16A16_UNORM,
            wgpu::TextureFormat::Rgba16Snorm => ash::vk::Format::R16G16B16A16_SNORM,
            wgpu::TextureFormat::Rgba16Float => ash::vk::Format::R16G16B16A16_SFLOAT,
            wgpu::TextureFormat::Rgba32Uint => ash::vk::Format::R32G32B32A32_UINT,
            wgpu::TextureFormat::Rgba32Sint => ash::vk::Format::R32G32B32A32_SINT,
            wgpu::TextureFormat::Rgba32Float => ash::vk::Format::R32G32B32A32_SFLOAT,
            wgpu::TextureFormat::Stencil8 => ash::vk::Format::S8_UINT,
            wgpu::TextureFormat::Depth16Unorm => ash::vk::Format::D16_UNORM,
            wgpu::TextureFormat::Depth24Plus => todo!(),
            wgpu::TextureFormat::Depth24PlusStencil8 => ash::vk::Format::D24_UNORM_S8_UINT,
            wgpu::TextureFormat::Depth32Float => ash::vk::Format::D32_SFLOAT,
            wgpu::TextureFormat::Depth32FloatStencil8 => ash::vk::Format::D32_SFLOAT_S8_UINT,
            wgpu::TextureFormat::NV12 => todo!(),
            wgpu::TextureFormat::Bc1RgbaUnorm => ash::vk::Format::BC1_RGBA_UNORM_BLOCK,
            wgpu::TextureFormat::Bc1RgbaUnormSrgb => ash::vk::Format::BC1_RGBA_SRGB_BLOCK,
            wgpu::TextureFormat::Bc2RgbaUnorm => ash::vk::Format::BC2_UNORM_BLOCK,
            wgpu::TextureFormat::Bc2RgbaUnormSrgb => ash::vk::Format::BC2_SRGB_BLOCK,
            wgpu::TextureFormat::Bc3RgbaUnorm => ash::vk::Format::BC3_UNORM_BLOCK,
            wgpu::TextureFormat::Bc3RgbaUnormSrgb => ash::vk::Format::BC3_SRGB_BLOCK,
            wgpu::TextureFormat::Bc4RUnorm => ash::vk::Format::BC4_UNORM_BLOCK,
            wgpu::TextureFormat::Bc4RSnorm => ash::vk::Format::BC4_SNORM_BLOCK,
            wgpu::TextureFormat::Bc5RgUnorm => ash::vk::Format::BC5_SNORM_BLOCK,
            wgpu::TextureFormat::Bc5RgSnorm => ash::vk::Format::BC5_SNORM_BLOCK,
            wgpu::TextureFormat::Bc6hRgbUfloat => ash::vk::Format::BC6H_UFLOAT_BLOCK,
            wgpu::TextureFormat::Bc6hRgbFloat => ash::vk::Format::BC6H_SFLOAT_BLOCK,
            wgpu::TextureFormat::Bc7RgbaUnorm => ash::vk::Format::BC7_UNORM_BLOCK,
            wgpu::TextureFormat::Bc7RgbaUnormSrgb => ash::vk::Format::BC7_SRGB_BLOCK,
            wgpu::TextureFormat::Etc2Rgb8Unorm => ash::vk::Format::ETC2_R8G8B8_UNORM_BLOCK,
            wgpu::TextureFormat::Etc2Rgb8UnormSrgb => ash::vk::Format::ETC2_R8G8B8_SRGB_BLOCK,
            wgpu::TextureFormat::Etc2Rgb8A1Unorm => ash::vk::Format::ETC2_R8G8B8A1_UNORM_BLOCK,
            wgpu::TextureFormat::Etc2Rgb8A1UnormSrgb => ash::vk::Format::ETC2_R8G8B8A1_SRGB_BLOCK,
            wgpu::TextureFormat::Etc2Rgba8Unorm => ash::vk::Format::ETC2_R8G8B8A8_UNORM_BLOCK,
            wgpu::TextureFormat::Etc2Rgba8UnormSrgb => ash::vk::Format::ETC2_R8G8B8A8_SRGB_BLOCK,
            wgpu::TextureFormat::EacR11Unorm => ash::vk::Format::EAC_R11_UNORM_BLOCK,
            wgpu::TextureFormat::EacR11Snorm => ash::vk::Format::EAC_R11_SNORM_BLOCK,
            wgpu::TextureFormat::EacRg11Unorm => ash::vk::Format::EAC_R11G11_UNORM_BLOCK,
            wgpu::TextureFormat::EacRg11Snorm => ash::vk::Format::EAC_R11G11_SNORM_BLOCK,
            wgpu::TextureFormat::Astc { .. } => todo!(),
        }
    }

    pub fn from_vk(format: ash::vk::Format) -> wgpu::TextureFormat {
        match format {
            ash::vk::Format::B8G8R8A8_UNORM => wgpu::TextureFormat::Bgra8Unorm,
            ash::vk::Format::B8G8R8A8_SRGB => wgpu::TextureFormat::Bgra8UnormSrgb,
            _ => wgpu::TextureFormat::Bgra8Unorm,
        }
    }
}

pub struct CompositeAlpha;
impl CompositeAlpha {
    pub fn to_vk(alpha_modes: &[wgpu::CompositeAlphaMode]) -> ash::vk::CompositeAlphaFlagsKHR {
        let mut flag = ash::vk::CompositeAlphaFlagsKHR::empty();
        for alpha_mode in alpha_modes {
            match alpha_mode {
                wgpu::CompositeAlphaMode::Auto => flag |= ash::vk::CompositeAlphaFlagsKHR::empty(),
                wgpu::CompositeAlphaMode::Opaque => flag |= ash::vk::CompositeAlphaFlagsKHR::OPAQUE,
                wgpu::CompositeAlphaMode::PreMultiplied => {
                    flag |= ash::vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED
                }
                wgpu::CompositeAlphaMode::PostMultiplied => {
                    flag |= ash::vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED
                }
                wgpu::CompositeAlphaMode::Inherit => {
                    flag |= ash::vk::CompositeAlphaFlagsKHR::INHERIT
                }
            }
        }

        flag
    }
}

pub struct TextureUsage;
impl TextureUsage {
    pub fn to_vk(usage: wgpu::TextureUsages) -> ash::vk::ImageUsageFlags {
        let mut flag = ash::vk::ImageUsageFlags::empty();
        if usage.contains(wgpu::TextureUsages::COPY_SRC) {
            flag |= ash::vk::ImageUsageFlags::TRANSFER_SRC;
        }
        if usage.contains(wgpu::TextureUsages::COPY_DST) {
            flag |= ash::vk::ImageUsageFlags::TRANSFER_DST;
        }
        if usage.contains(wgpu::TextureUsages::TEXTURE_BINDING) {
            flag |= ash::vk::ImageUsageFlags::SAMPLED;
        }
        if usage.contains(wgpu::TextureUsages::STORAGE_BINDING) {
            flag |= ash::vk::ImageUsageFlags::STORAGE;
        }
        if usage.contains(wgpu::TextureUsages::RENDER_ATTACHMENT) {
            flag |= ash::vk::ImageUsageFlags::COLOR_ATTACHMENT;
        }
        if usage.contains(wgpu::TextureUsages::STORAGE_ATOMIC) {
            // TODO: 適切な変換先を決定する
        }

        flag
    }
}
