use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use glam::{Mat4, Vec3};

use crate::render::vertex::{Vertex, UniformData};

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    solid_pipeline: wgpu::RenderPipeline,
    star_pipeline: wgpu::RenderPipeline,
    depth_texture_view: wgpu::TextureView,
    
    static_world_vertex_buffer: wgpu::Buffer,
    static_world_vertex_count: u32,
    
    cube_vertex_buffer: wgpu::Buffer,
    cube_vertex_count: u32,
    
    star_vertex_buffer: wgpu::Buffer,
    star_vertex_count: u32,
    
    world_uniform_buffer: wgpu::Buffer,
    world_bind_group: wgpu::BindGroup,
    
    cube_uniform_buffer: wgpu::Buffer,
    cube_bind_group: wgpu::BindGroup,
    
    projection_matrix: Mat4,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false,
        })).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter().copied().find(|f| f.is_srgb()).unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, format: surface_format, width: size.width, height: size.height,
            present_mode: wgpu::PresentMode::Fifo, alpha_mode: surface_caps.alpha_modes[0], view_formats: vec![], desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let depth_texture_view = create_depth_texture(&device, &config);
        
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0, visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, count: None,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
            }],
            label: None,
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None, bind_group_layouts: &[&uniform_bind_group_layout], push_constant_ranges: &[],
        });

        let solid_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Solid"), layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[Vertex::desc()], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(config.format.into())], compilation_options: Default::default() }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: Some(wgpu::Face::Back), ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState::default(), multiview: None,
        });

        let star_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Stars"), layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[Vertex::desc()], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(config.format.into())], compilation_options: Default::default() }),
            primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::PointList, ..Default::default() },
            depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: false, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState::default(), multiview: None,
        });

        let mut static_world_verts = crate::floor::create_vertices();
        static_world_verts.extend(crate::world::high_building::create_vertices());
        static_world_verts.extend(crate::world::pyramid::create_vertices());
        
        let cube_verts = crate::player::create_vertices();
        let star_verts = crate::sky::create_vertices();

        let static_world_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&static_world_verts), usage: wgpu::BufferUsages::VERTEX });
        let cube_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&cube_verts), usage: wgpu::BufferUsages::VERTEX });
        let star_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&star_verts), usage: wgpu::BufferUsages::VERTEX });

        let matrix_size = std::mem::size_of::<UniformData>() as u64;
        let world_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: matrix_size, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let world_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &uniform_bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: world_uniform_buffer.as_entire_binding() }], label: None });

        let cube_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: matrix_size, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let cube_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &uniform_bind_group_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: cube_uniform_buffer.as_entire_binding() }], label: None });

        let aspect = size.width as f32 / size.height as f32;
        let projection_matrix = Mat4::perspective_rh(45.0_f32.to_radians(), aspect, 0.1, 2000.0); 

        Self {
            surface, device, queue, config, size, solid_pipeline, star_pipeline, depth_texture_view,
            static_world_vertex_buffer, static_world_vertex_count: static_world_verts.len() as u32,
            cube_vertex_buffer, cube_vertex_count: cube_verts.len() as u32,
            star_vertex_buffer, star_vertex_count: star_verts.len() as u32,
            world_uniform_buffer, world_bind_group, cube_uniform_buffer, cube_bind_group, projection_matrix,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width; self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture_view = create_depth_texture(&self.device, &self.config);
            self.projection_matrix = Mat4::perspective_rh(45.0_f32.to_radians(), self.size.width as f32 / self.size.height as f32, 0.1, 2000.0);
        }
    }

    // Now accepts `player_yaw` separately from `camera_yaw`
    pub fn update_matrices(&self, player_pos: Vec3, player_yaw: f32, camera_yaw: f32, camera_pitch: f32, camera_dist: f32) {
        let cam_offset = Vec3::new(camera_yaw.sin() * camera_pitch.cos() * camera_dist, camera_pitch.sin() * camera_dist, camera_yaw.cos() * camera_pitch.cos() * camera_dist);
        let view_proj = self.projection_matrix * Mat4::look_at_rh(player_pos + cam_offset, player_pos, Vec3::Y);

        let light_dir = [0.8, 1.0, 0.5, 0.0]; 
        let light_color = [1.0, 1.0, 1.0, 0.4]; 

        let world_model = Mat4::IDENTITY;
        self.queue.write_buffer(&self.world_uniform_buffer, 0, bytemuck::cast_slice(&[UniformData { 
            mvp_matrix: (view_proj * world_model).to_cols_array_2d(),
            model_matrix: world_model.to_cols_array_2d(),
            light_dir, light_color
        }]));
        
        // Rotate the cube based on the PLAYER'S yaw, not the camera's
        let cube_model = Mat4::from_translation(player_pos + Vec3::new(0.0, 0.5, 0.0)) * Mat4::from_rotation_y(player_yaw);
        self.queue.write_buffer(&self.cube_uniform_buffer, 0, bytemuck::cast_slice(&[UniformData { 
            mvp_matrix: (view_proj * cube_model).to_cols_array_2d(),
            model_matrix: cube_model.to_cols_array_2d(),
            light_dir, light_color
        }]));
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(crate::sky::colors::SKY_BLACK), store: wgpu::StoreOp::Store } })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth_texture_view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }),
                timestamp_writes: None, occlusion_query_set: None,
            });

            pass.set_pipeline(&self.solid_pipeline);
            
            pass.set_bind_group(0, &self.world_bind_group, &[]);
            pass.set_vertex_buffer(0, self.static_world_vertex_buffer.slice(..));
            pass.draw(0..self.static_world_vertex_count, 0..1);

            pass.set_bind_group(0, &self.cube_bind_group, &[]);
            pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
            pass.draw(0..self.cube_vertex_count, 0..1);

            pass.set_pipeline(&self.star_pipeline);
            pass.set_bind_group(0, &self.world_bind_group, &[]); 
            pass.set_vertex_buffer(0, self.star_vertex_buffer.slice(..));
            pass.draw(0..self.star_vertex_count, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
    device.create_texture(&wgpu::TextureDescriptor {
        label: None, size: wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 },
        mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[],
    }).create_view(&wgpu::TextureViewDescriptor::default())
}