use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;
use glam::{Mat4, Vec3};

use crate::render::vertex::{Vertex, UniformData, PointLight};
use crate::render::texture::Texture;
use crate::sky::SkyboxVertex;

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>, 
    device: wgpu::Device, 
    queue: wgpu::Queue, 
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    
    solid_pipeline: wgpu::RenderPipeline, 
    shadow_pipeline: wgpu::RenderPipeline,
    skybox_pipeline: wgpu::RenderPipeline, 
    
    depth_texture_view: wgpu::TextureView, 
    shadow_map: Texture,
    
    floor_buffer: wgpu::Buffer, floor_count: u32, floor_tex: Texture,
    building_buffer: wgpu::Buffer, building_count: u32, building_tex: Texture,
    pyramid_buffer: wgpu::Buffer, pyramid_count: u32, pyramid_tex: Texture,
    street_light_buffer: wgpu::Buffer, street_light_count: u32, street_light_tex: Texture, 
    cube_buffer: wgpu::Buffer, cube_count: u32, 
    player_side_tex: Texture, player_top_tex: Texture,
    
    skybox_buffer: wgpu::Buffer, skybox_count: u32,
    
    world_uniform_buffer: wgpu::Buffer, world_bind_group: wgpu::BindGroup,
    cube_uniform_buffer: wgpu::Buffer, cube_bind_group: wgpu::BindGroup,
    projection_matrix: Mat4, light_projection_matrix: Mat4,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() });
        let surface = instance.create_surface(window).unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false })).unwrap();
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
        let shadow_shader = device.create_shader_module(wgpu::include_wgsl!("shadow.wgsl")); 
        let skybox_shader = device.create_shader_module(wgpu::include_wgsl!("skybox.wgsl"));

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None }], label: None });
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Float { filterable: true } }, count: None }, wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering), count: None }], label: None });
        let shadow_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { entries: &[wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Texture { multisampled: false, view_dimension: wgpu::TextureViewDimension::D2, sample_type: wgpu::TextureSampleType::Depth }, count: None }, wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::FRAGMENT, ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison), count: None }], label: None });

        let floor_tex = Texture::from_bytes(&device, &queue, crate::world::prefabs::static_objs::grounds::ground01::TEXTURE_BYTES, "floor", &texture_layout).unwrap();
        let building_tex = Texture::from_bytes(&device, &queue, crate::world::prefabs::static_objs::buildings::building01::TEXTURE_BYTES, "building", &texture_layout).unwrap();
        let pyramid_tex = Texture::from_bytes(&device, &queue, crate::world::prefabs::static_objs::buildings::building02::TEXTURE_BYTES, "pyramid", &texture_layout).unwrap();
        let street_light_tex = Texture::from_bytes(&device, &queue, crate::world::prefabs::static_objs::streetlights::streetlight01::TEXTURE_BYTES, "street_light", &texture_layout).unwrap(); 
        let player_side_tex = Texture::from_bytes(&device, &queue, include_bytes!("../../assets/player_side.png"), "player_side", &texture_layout).unwrap();
        let player_top_tex = Texture::from_bytes(&device, &queue, include_bytes!("../../assets/player_top.png"), "player_top", &texture_layout).unwrap();
        let shadow_map = Texture::create_shadow_map(&device, &shadow_layout);

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: None, bind_group_layouts: &[&uniform_layout, &texture_layout, &shadow_layout], push_constant_ranges: &[] });
        let shadow_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("Shadow Layout"), bind_group_layouts: &[&uniform_layout], push_constant_ranges: &[] });
        let skybox_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { label: Some("Skybox Layout"), bind_group_layouts: &[&uniform_layout], push_constant_ranges: &[] }); 

        let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("Shadow"), layout: Some(&shadow_pipeline_layout), vertex: wgpu::VertexState { module: &shadow_shader, entry_point: "vs_main", buffers: &[Vertex::desc()], compilation_options: Default::default() }, fragment: None, primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: Some(wgpu::Face::Back), ..Default::default() }, depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::LessEqual, stencil: Default::default(), bias: wgpu::DepthBiasState { constant: 2, slope_scale: 2.0, clamp: 0.0 } }), multisample: wgpu::MultisampleState::default(), multiview: None });
        let solid_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { label: Some("Solid"), layout: Some(&render_pipeline_layout), vertex: wgpu::VertexState { module: &shader, entry_point: "vs_main", buffers: &[Vertex::desc()], compilation_options: Default::default() }, fragment: Some(wgpu::FragmentState { module: &shader, entry_point: "fs_main", targets: &[Some(config.format.into())], compilation_options: Default::default() }), primitive: wgpu::PrimitiveState { topology: wgpu::PrimitiveTopology::TriangleList, cull_mode: Some(wgpu::Face::Back), ..Default::default() }, depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: true, depth_compare: wgpu::CompareFunction::Less, stencil: Default::default(), bias: Default::default() }), multisample: wgpu::MultisampleState::default(), multiview: None });
        
        let skybox_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
            label: Some("Skybox"), 
            layout: Some(&skybox_pipeline_layout), 
            vertex: wgpu::VertexState { module: &skybox_shader, entry_point: "vs_main", buffers: &[SkyboxVertex::desc()], compilation_options: Default::default() }, 
            fragment: Some(wgpu::FragmentState { module: &skybox_shader, entry_point: "fs_main", targets: &[Some(config.format.into())], compilation_options: Default::default() }), 
            primitive: wgpu::PrimitiveState { 
                topology: wgpu::PrimitiveTopology::TriangleList, 
                cull_mode: None, 
                ..Default::default() 
            }, 
            depth_stencil: Some(wgpu::DepthStencilState { 
                format: wgpu::TextureFormat::Depth32Float, 
                depth_write_enabled: false, 
                depth_compare: wgpu::CompareFunction::LessEqual, 
                stencil: Default::default(), bias: Default::default() 
            }), 
            multisample: wgpu::MultisampleState::default(), multiview: None 
        });

        let world_state = crate::world::WorldState::new();
        let f_verts = world_state.get_ground_vertices();
        let b_verts = world_state.get_building01_vertices();
        let p_verts = world_state.get_building02_vertices();
        let sl_verts = world_state.get_streetlight_vertices(); 
        let c_verts = crate::player::create_vertices();
        let sky_verts = crate::sky::create_skybox_cube();

        let floor_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&f_verts), usage: wgpu::BufferUsages::VERTEX });
        let building_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&b_verts), usage: wgpu::BufferUsages::VERTEX });
        let pyramid_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&p_verts), usage: wgpu::BufferUsages::VERTEX });
        let street_light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&sl_verts), usage: wgpu::BufferUsages::VERTEX }); 
        let cube_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&c_verts), usage: wgpu::BufferUsages::VERTEX });
        let skybox_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: None, contents: bytemuck::cast_slice(&sky_verts), usage: wgpu::BufferUsages::VERTEX });

        let matrix_size = std::mem::size_of::<UniformData>() as u64;
        let world_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: matrix_size, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let world_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &uniform_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: world_uniform_buffer.as_entire_binding() }], label: None });

        let cube_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor { label: None, size: matrix_size, usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let cube_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { layout: &uniform_layout, entries: &[wgpu::BindGroupEntry { binding: 0, resource: cube_uniform_buffer.as_entire_binding() }], label: None });

        let aspect = size.width as f32 / size.height as f32;
        let projection_matrix = Mat4::perspective_rh(45.0_f32.to_radians(), aspect, 0.1, 2000.0); 
        let light_projection_matrix = Mat4::orthographic_rh(-200.0, 200.0, -200.0, 200.0, 1.0, 2000.0);

        Self {
            surface, device, queue, config, size, solid_pipeline, shadow_pipeline, skybox_pipeline, depth_texture_view, shadow_map,
            floor_buffer, floor_count: f_verts.len() as u32, floor_tex,
            building_buffer, building_count: b_verts.len() as u32, building_tex,
            pyramid_buffer, pyramid_count: p_verts.len() as u32, pyramid_tex,
            street_light_buffer, street_light_count: sl_verts.len() as u32, street_light_tex,
            cube_buffer, cube_count: c_verts.len() as u32, player_side_tex, player_top_tex,
            skybox_buffer, skybox_count: sky_verts.len() as u32,
            world_uniform_buffer, world_bind_group, cube_uniform_buffer, cube_bind_group, projection_matrix, light_projection_matrix,
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

    // NEW: Accepts is_freeform directly from the Game Loop
    pub fn update_matrices(&self, player_pos: Vec3, player_yaw: f32, camera: &crate::camera::Camera, is_day: bool, is_flashlight_on: bool, is_freeform: bool) {
        let forward_dir = Vec3::new(
            -camera.yaw.sin() * camera.pitch.cos(),
            -camera.pitch.sin(),
            -camera.yaw.cos() * camera.pitch.cos()
        ).normalize();
        
        let view_proj = if is_freeform {
            self.projection_matrix * Mat4::look_at_rh(camera.pos, camera.pos + forward_dir, Vec3::Y)
        } else {
            let cam_offset = Vec3::new(camera.yaw.sin() * camera.pitch.cos() * camera.distance, camera.pitch.sin() * camera.distance, camera.yaw.cos() * camera.pitch.cos() * camera.distance);
            self.projection_matrix * Mat4::look_at_rh(player_pos + cam_offset, player_pos, Vec3::Y)
        };

        let az = crate::sky::config::SUN_AZIMUTH.to_radians();
        let el = crate::sky::config::SUN_ELEVATION.to_radians();
        let sun_dir_vec = Vec3::new(
            el.cos() * az.sin(),
            el.sin(),
            el.cos() * az.cos()
        ).normalize();
        let sun_dir = [sun_dir_vec.x, sun_dir_vec.y, sun_dir_vec.z, 0.0]; 
        
        let s_col = crate::sky::config::SUN_COLOR;
        let sun_color = if is_day { [s_col[0], s_col[1], s_col[2], crate::sky::config::SUN_INTENSITY_DAY] } else { [0.0, 0.0, 0.0, 0.0] }; 
        let ambient_color = if is_day { [0.3, 0.3, 0.4, 1.0] } else { [0.05, 0.05, 0.1, 1.0] }; 

        let cam_pos = if is_freeform {
            camera.pos
        } else {
            let cam_offset = Vec3::new(camera.yaw.sin() * camera.pitch.cos() * camera.distance, camera.pitch.sin() * camera.distance, camera.yaw.cos() * camera.pitch.cos() * camera.distance);
            player_pos + cam_offset
        };

        let light_distance = 600.0;
        let light_target = if is_freeform { camera.pos } else { player_pos };
        let light_pos = light_target + (sun_dir_vec * light_distance);
        let light_view = Mat4::look_at_rh(light_pos, light_target, Vec3::Y);
        let light_mvp_matrix = (self.light_projection_matrix * light_view).to_cols_array_2d();

        let point_lights = [
            PointLight { position: [348.5, 9.8, 550.0, 0.0], color: [1.0, 0.8, 0.4, 50.0] },
            PointLight { position: [351.5, 9.8, 550.0, 0.0], color: [1.0, 0.8, 0.4, 50.0] }, 
        ];

        let flashlight_pos = [cam_pos.x, cam_pos.y, cam_pos.z, 1.0];
        let flashlight_dir = [forward_dir.x, forward_dir.y, forward_dir.z, 0.0];
        let flashlight_color = if is_flashlight_on { [1.0, 0.95, 0.9, 150.0] } else { [0.0, 0.0, 0.0, 0.0] };

        let mut view_matrix = if is_freeform {
            Mat4::look_at_rh(camera.pos, camera.pos + forward_dir, Vec3::Y)
        } else {
            let cam_offset = Vec3::new(camera.yaw.sin() * camera.pitch.cos() * camera.distance, camera.pitch.sin() * camera.distance, camera.yaw.cos() * camera.pitch.cos() * camera.distance);
            Mat4::look_at_rh(player_pos + cam_offset, player_pos, Vec3::Y)
        };
        view_matrix.w_axis = glam::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let sky_view_proj = self.projection_matrix * view_matrix;

        let z_c = crate::sky::config::SKY_ZENITH_DAY;
        let h_c = crate::sky::config::SKY_HORIZON_DAY;
        let n_c = crate::sky::config::SKY_NIGHT;
        let sky_zenith = [z_c[0], z_c[1], z_c[2], 1.0];
        let sky_horizon = [h_c[0], h_c[1], h_c[2], 1.0];
        let sky_night = [n_c[0], n_c[1], n_c[2], 1.0];

        let world_model = Mat4::IDENTITY;
        self.queue.write_buffer(&self.world_uniform_buffer, 0, bytemuck::cast_slice(&[UniformData { 
            mvp_matrix: (view_proj * world_model).to_cols_array_2d(), 
            model_matrix: world_model.to_cols_array_2d(), 
            light_mvp_matrix, sun_dir, sun_color, ambient_color, point_lights,
            flashlight_pos, flashlight_dir, flashlight_color,
            sky_mvp_matrix: sky_view_proj.to_cols_array_2d(),
            sky_zenith, sky_horizon, sky_night
        }]));
        
        let cube_model = Mat4::from_translation(player_pos + Vec3::new(0.0, 0.5, 0.0)) * Mat4::from_rotation_y(player_yaw);
        self.queue.write_buffer(&self.cube_uniform_buffer, 0, bytemuck::cast_slice(&[UniformData { 
            mvp_matrix: (view_proj * cube_model).to_cols_array_2d(), 
            model_matrix: cube_model.to_cols_array_2d(), 
            light_mvp_matrix, sun_dir, sun_color, ambient_color, point_lights,
            flashlight_pos, flashlight_dir, flashlight_color,
            sky_mvp_matrix: sky_view_proj.to_cols_array_2d(),
            sky_zenith, sky_horizon, sky_night
        }]));
    }

    // NEW: Accepts is_freeform directly from the Game Loop
    pub fn render(&mut self, _is_day: bool, is_freeform: bool) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let clear_color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }; 

        {
            let mut shadow_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("Shadow Pass"), color_attachments: &[], depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.shadow_map.view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
            shadow_pass.set_pipeline(&self.shadow_pipeline);
            shadow_pass.set_bind_group(0, &self.world_bind_group, &[]);
            shadow_pass.set_vertex_buffer(0, self.floor_buffer.slice(..)); shadow_pass.draw(0..self.floor_count, 0..1);
            shadow_pass.set_vertex_buffer(0, self.building_buffer.slice(..)); shadow_pass.draw(0..self.building_count, 0..1);
            shadow_pass.set_vertex_buffer(0, self.pyramid_buffer.slice(..)); shadow_pass.draw(0..self.pyramid_count, 0..1);
            shadow_pass.set_vertex_buffer(0, self.street_light_buffer.slice(..)); shadow_pass.draw(0..self.street_light_count, 0..1);
            
            if !is_freeform {
                shadow_pass.set_bind_group(0, &self.cube_bind_group, &[]);
                shadow_pass.set_vertex_buffer(0, self.cube_buffer.slice(..));
                shadow_pass.draw(0..self.cube_count, 0..1);
            }
        }

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { label: Some("Main Pass"), color_attachments: &[Some(wgpu::RenderPassColorAttachment { view: &view, resolve_target: None, ops: wgpu::Operations { load: wgpu::LoadOp::Clear(clear_color), store: wgpu::StoreOp::Store } })], depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { view: &self.depth_texture_view, depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }), stencil_ops: None }), timestamp_writes: None, occlusion_query_set: None });
            
            pass.set_pipeline(&self.skybox_pipeline);
            pass.set_bind_group(0, &self.world_bind_group, &[]);
            pass.set_vertex_buffer(0, self.skybox_buffer.slice(..));
            pass.draw(0..self.skybox_count, 0..1);

            pass.set_pipeline(&self.solid_pipeline);
            pass.set_bind_group(2, &self.shadow_map.bind_group, &[]);
            
            pass.set_bind_group(0, &self.world_bind_group, &[]);
            pass.set_bind_group(1, &self.floor_tex.bind_group, &[]); pass.set_vertex_buffer(0, self.floor_buffer.slice(..)); pass.draw(0..self.floor_count, 0..1);
            pass.set_bind_group(1, &self.building_tex.bind_group, &[]); pass.set_vertex_buffer(0, self.building_buffer.slice(..)); pass.draw(0..self.building_count, 0..1);
            pass.set_bind_group(1, &self.pyramid_tex.bind_group, &[]); pass.set_vertex_buffer(0, self.pyramid_buffer.slice(..)); pass.draw(0..self.pyramid_count, 0..1);
            pass.set_bind_group(1, &self.street_light_tex.bind_group, &[]); pass.set_vertex_buffer(0, self.street_light_buffer.slice(..)); pass.draw(0..self.street_light_count, 0..1);

            if !is_freeform {
                pass.set_bind_group(0, &self.cube_bind_group, &[]);
                pass.set_vertex_buffer(0, self.cube_buffer.slice(..));
                pass.set_bind_group(1, &self.player_top_tex.bind_group, &[]); pass.draw(0..6, 0..1); 
                pass.set_bind_group(1, &self.player_side_tex.bind_group, &[]); pass.draw(6..30, 0..1); 
                pass.set_bind_group(1, &self.player_top_tex.bind_group, &[]); pass.draw(30..36, 0..1); 
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}

fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
    device.create_texture(&wgpu::TextureDescriptor { label: None, size: wgpu::Extent3d { width: config.width, height: config.height, depth_or_array_layers: 1 }, mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2, format: wgpu::TextureFormat::Depth32Float, usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, view_formats: &[] }).create_view(&wgpu::TextureViewDescriptor::default())
}