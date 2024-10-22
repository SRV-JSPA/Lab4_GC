
use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    planeta_gaseoso(fragment, uniforms)
    // dalmata_shader(fragment, uniforms)
    // cloud_shader(fragment, uniforms)
    // cellular_shader(fragment, uniforms)
    // lava_shader(fragment, uniforms)
}

fn ruido_perlin(x: f32, y: f32) -> f32 {
  (x.sin() * y.cos()) * 0.5
}

fn planeta_gaseoso(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let color3 = Color::new(245, 245, 220);    
  let color1 = Color::new(255, 255, 255);    
  let color2 = Color::new(173, 216, 230);    
  let color4 = Color::new(25, 25, 112);      
  let color5 = Color::new(112, 128, 144);    

  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let tiempo = (uniforms.time as f32) * 0.01;  

  let frecuencia = 10.0;
  let distancia = (x * x + y * y).sqrt();

  let ruido = ruido_perlin(x * 0.5 + tiempo, y * 0.5);  

  let angulo = tiempo * 0.5;  


  let patron1 = ((distancia + ruido) * 7.0 * frecuencia + (y + ruido) * 5.0 + angulo).sin() * 0.5 + 0.5;
  let patron2 = ((distancia + ruido) * 5.0 * frecuencia - (y + ruido) * 8.0 + PI / 3.0 + angulo).sin() * 0.5 + 0.5;
  let patron3 = ((distancia + ruido) * 6.0 * frecuencia + (x + ruido) * 4.0 + 2.0 * PI / 3.0 + angulo).sin() * 0.5 + 0.5;

  let mut color_final = color1.lerp(&color2, patron1);
  color_final = color_final.lerp(&color3, patron2);
  color_final = color_final.lerp(&color4, patron3);
  color_final = color_final.lerp(&color5, patron2);

  color_final * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;
  
    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);
  
    let random_number = rng.gen_range(0..=100);
  
    let black_or_white = if random_number < 50 {
      Color::new(0, 0, 0)
    } else {
      Color::new(255, 255, 255)
    };
  
    black_or_white * fragment.intensity
}
  
fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    let noise_value = uniforms.noise.get_noise_2d(
      (x + ox) * zoom,
      (y + oy) * zoom,
    );
  
    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); 
    let base_color = Color::new(0, 0, 0); 
  
    let noise_color = if noise_value < spot_threshold {
      spot_color
    } else {
      base_color
    };
  
    noise_color * fragment.intensity
}
  
fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;  
    let ox = 100.0; 
    let oy = 100.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;
  
    let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);
  
   
    let cloud_threshold = 0.5; 
    let cloud_color = Color::new(255, 255, 255); 
    let sky_color = Color::new(30, 97, 145); 
  
    
    let noise_color = if noise_value > cloud_threshold {
      cloud_color
    } else {
      sky_color
    };
  
    noise_color * fragment.intensity
}
  
fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0;  
    let ox = 50.0;    
    let oy = 50.0;    
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
  
    
    let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();
  
    
    let cell_color_1 = Color::new(85, 107, 47);   
    let cell_color_2 = Color::new(124, 252, 0);   
    let cell_color_3 = Color::new(34, 139, 34);   
    let cell_color_4 = Color::new(173, 255, 47);  
  
    let color_final = if cell_noise_value < 0.15 {
      cell_color_1
    } else if cell_noise_value < 0.7 {
      cell_color_2
    } else if cell_noise_value < 0.75 {
      cell_color_3
    } else {
      cell_color_4
    };
  
    color_final * fragment.intensity
}
  
fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let bright_color = Color::new(255, 240, 0); 
    let dark_color = Color::new(130, 20, 0);   
  
    let position = Vec3::new(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
      fragment.depth
    );
  
    let base_frecuencia = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;
  
    let pulsate = (t * base_frecuencia).sin() * pulsate_amplitude;
  
    let zoom = 1000.0; 
    let noise_value1 = uniforms.noise.get_noise_3d(
      position.x * zoom,
      position.y * zoom,
      (position.z + pulsate) * zoom
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
      (position.x + 1000.0) * zoom,
      (position.y + 1000.0) * zoom,
      (position.z + 1000.0 + pulsate) * zoom
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5;  
  
   
    let color = dark_color.lerp(&bright_color, noise_value);
  
    color * fragment.intensity
}