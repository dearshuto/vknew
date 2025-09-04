use std::ops::Range;

use crate::wgpu::details::compatibility_layer::{BufferId, CommandBufferId, Context};

use super::{Accessor, RenderPipelineId};

pub enum Command {
    Begin,
    End,
    BeginRendering(BeginRenderingParameters),
    EndRendering,
    BindRenderPipeline(RenderPipelineId),
    BindVertexBuffers(BindVertexBuffersParameters),
    Draw(DrawParameters),
}

pub struct BeginRenderingParameters {
    pub clear: [f32; 4],
}

pub struct DrawParameters {
    pub vertices: Range<u32>,
    pub instances: Range<u32>,
}

pub struct BindVertexBuffersParameters {
    pub first_binding: u32,
    pub count: u32,
    pub buffers: [BufferId; 8],
    pub offsets: [u64; 8],
}

pub struct CommandEmulator;

impl CommandEmulator {
    pub fn push_draw_command(
        render_pass: &mut wgpu::RenderPass,
        command_buffer: ash::vk::CommandBuffer,
    ) {
        let (id, instance_handle) = Accessor::from(command_buffer)
            .peek_return(|context: &_| (context.id, context.instance_handle));
        Accessor::from(instance_handle).peek(|context: &_| {
            let Some(context) = &context.context else {
                return;
            };

            let Some(command_list) = context.command_table.get(&id) else {
                return;
            };

            Self::func(render_pass, &mut command_list.iter(), &context);
        });
    }

    pub fn emulate(
        id: CommandBufferId,
        context: &Context,
        mut command_encoder: wgpu::CommandEncoder,
    ) -> wgpu::CommandEncoder {
        let Some(command_list) = context.command_table.get(&id) else {
            return command_encoder;
        };
        let mut command_iterator = command_list.iter();

        while let Some(command) = command_iterator.next() {
            match command {
                Command::Begin => {}
                Command::End => {}
                Command::BeginRendering(begin_rendering_parameters) => {
                    Self::push_draw_command_impl(
                        &mut command_iterator,
                        context,
                        begin_rendering_parameters,
                        &mut command_encoder,
                    )
                }
                Command::EndRendering => {}
                // 描画コマンドで処理されるはずのコマンド
                Command::BindRenderPipeline(_) => todo!(),
                Command::BindVertexBuffers(_) => todo!(),
                Command::Draw(_) => todo!(),
            }
        }

        command_encoder
    }

    fn push_draw_command_impl<'a, I>(
        command_iterator: &mut I,
        context: &Context,
        begin_rendering_parameters: &BeginRenderingParameters,
        command_encoder: &mut wgpu::CommandEncoder,
    ) where
        I: Iterator<Item = &'a Command>,
    {
        let Some(surface_texture) = &context.surface_texture else {
            return;
        };

        let clear = begin_rendering_parameters.clear;
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default()),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: clear[0] as f64,
                        g: clear[1] as f64,
                        b: clear[2] as f64,
                        a: clear[2] as f64,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        Self::func(&mut render_pass, command_iterator, context);
        drop(render_pass);
    }

    fn func<'a, I>(render_pass: &mut wgpu::RenderPass, command_iterator: &mut I, context: &Context)
    where
        I: Iterator<Item = &'a Command>,
    {
        while let Some(command) = command_iterator.next() {
            match command {
                Command::EndRendering => {
                    return;
                }
                Command::BindRenderPipeline(render_pipeline_id) => {
                    let Some(render_pipeline) =
                        context.render_pipeline_table.get(render_pipeline_id)
                    else {
                        continue;
                    };

                    render_pass.set_pipeline(render_pipeline);
                }
                Command::BindVertexBuffers(parameters) => {
                    for index in
                        parameters.first_binding..(parameters.first_binding + parameters.count)
                    {
                        let id = parameters.buffers[index as usize];
                        let Some(buffer) = context.buffer_table.get(&id) else {
                            continue;
                        };

                        let offset = parameters.offsets[index as usize];
                        render_pass.set_vertex_buffer(index, buffer.slice(offset..));
                    }
                }
                Command::Draw(draw_parameters) => {
                    render_pass.draw(
                        draw_parameters.vertices.clone(),
                        draw_parameters.instances.clone(),
                    );
                }
                Command::Begin => {}
                Command::End => {}
                // 描画コマンド構築中は来ないはずのコマンド
                Command::BeginRendering(_) => todo!(),
            }
        }
    }
}
