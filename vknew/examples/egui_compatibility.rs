use std::sync::Arc;

use eframe::egui_wgpu::Callback;

#[path = "common/mod.rs"]
mod common;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "vknew on egui",
            options,
            Box::new(|cc| Ok(Box::new(App::new(cc)))),
        )
        .unwrap();
    }
}

struct App {
    triangle_adapter: TriangleAdapter,
}

impl App {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let wgpu_state = cc.wgpu_render_state.as_ref().unwrap();
        let info = vknew::wgpu::WasmCompatibilityCreateInfo::from_instance(
            wgpu_state.device.clone(),
            wgpu_state.queue.clone(),
            wgpu_state.adapter.clone(),
        );

        let triangle = Triangle::new(wgpu_state.target_format, info);
        Self {
            triangle_adapter: TriangleAdapter(Arc::new(triangle)),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::SidePanel::left("Left").show(ctx, |ui| {
            if ui.button("Hello World").clicked() {
                println!("Clicked");
            }
        });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            eframe::egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let (rect, _response) = ui.allocate_exact_size(
                    eframe::egui::vec2(700.0, 700.0),
                    eframe::egui::Sense::drag(),
                );

                let callback = Callback::new_paint_callback(rect, self.triangle_adapter.clone());
                ui.painter().add(callback);
            });
        });
    }
}

#[allow(unused)]
struct Triangle {
    instance: ash::Instance,
    device: ash::Device,
    queue: ash::vk::Queue,
    shader_module: ash::vk::ShaderModule,
    pipeline: ash::vk::Pipeline,
    command_pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,
}

impl Triangle {
    pub fn new(
        target_format: wgpu::TextureFormat,
        mut create_info: vknew::wgpu::WasmCompatibilityCreateInfo,
    ) -> Self {
        let entry = unsafe { ash::Entry::from_static_fn(vknew::wgpu::get_static_fn()) };
        let instance = {
            let create_info = ash::vk::InstanceCreateInfo::default().push_next(&mut create_info);
            unsafe { entry.create_instance(&create_info, None) }.unwrap()
        };

        let physical_device = unsafe { instance.enumerate_physical_devices() }.unwrap()[0];
        let device = {
            let create_info = ash::vk::DeviceCreateInfo::default();
            unsafe { instance.create_device(physical_device, &create_info, None) }.unwrap()
        };

        let queue = unsafe { device.get_device_queue(0, 0) };

        let (vs_shader_module, fs_shader_module) = {
            let vs_source = r"
            #version 450
            void main()
            {
                if (gl_VertexIndex == 0)
                {
                    gl_Position = vec4(0.0, 0.5, 0.0, 1.0);
                }
                else if (gl_VertexIndex == 1)
                {
                    gl_Position = vec4(-0.5, -0.5, 0.0, 1.0);   
                }
                else
                {
                    gl_Position = vec4(0.5, -0.5, 0.0, 1.0);   
                }
            }";
            let fs_source = r"
            #version 450
            layout(location = 0) out vec4 o_Color;
            void main()
            {
                o_Color = vec4(1.0);
            }";

            let vs_code = common::convert_shader(&vs_source, naga::ShaderStage::Vertex);
            let fs_code = common::convert_shader(&fs_source, naga::ShaderStage::Fragment);

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
            let vertex_input_state = ash::vk::PipelineVertexInputStateCreateInfo::default();
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
            let color_attachment_formatts = [vknew::wgpu::TextureFormat::to_vk(target_format)];
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

        let command_pool = {
            let create_info = ash::vk::CommandPoolCreateInfo::default();
            unsafe { device.create_command_pool(&create_info, None) }.unwrap()
        };

        let command_buffer = {
            let allocate_info = ash::vk::CommandBufferAllocateInfo::default()
                .command_pool(command_pool)
                .level(ash::vk::CommandBufferLevel::SECONDARY)
                .command_buffer_count(1);
            unsafe { device.allocate_command_buffers(&allocate_info) }.unwrap()[0]
        };

        unsafe {
            let begin_info = ash::vk::CommandBufferBeginInfo::default();
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .unwrap();

            device.cmd_bind_pipeline(
                command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                pipeline,
            );

            device.cmd_draw(
                command_buffer,
                3, /*vertex_count*/
                1, /*instance_count*/
                0, /*first_vertex*/
                0, /*first_instance*/
            );

            device.end_command_buffer(command_buffer).unwrap();
        }

        Self {
            instance,
            device,
            queue,
            shader_module: ash::vk::ShaderModule::null(),
            command_pool,
            command_buffer,
            pipeline: ash::vk::Pipeline::null(),
        }
    }
}

impl Drop for Triangle {
    fn drop(&mut self) {
        let device = &self.device;
        unsafe {
            device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

#[derive(Clone)]
struct TriangleAdapter(Arc<Triangle>);

impl eframe::egui_wgpu::CallbackTrait for TriangleAdapter {
    fn paint(
        &self,
        _info: eframe::egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'static>,
        _callback_resources: &eframe::egui_wgpu::CallbackResources,
    ) {
        let device = self.0.device.handle();
        let command_buffer = self.0.command_buffer;
        vknew::wgpu::push_draw_command(render_pass, device, command_buffer);
    }
}
