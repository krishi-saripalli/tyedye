use std::time::Instant;

use glium::uniforms::EmptyUniforms;
use glium::uniforms::UniformsStorage;
use rand::Rng;

fn generate_random_float(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..max);
}

pub fn get_vertex_shader() -> &'static str {
    let vertex_shader = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position,0.0, 1.0);
        }
    "#;
    return vertex_shader;
}

pub fn get_fragment_shader() -> &'static str {
    let fragment_shader = r#"
        #version 140

    vec3 palette( float t ) {
        vec3 a = vec3(0.8, 0.2, 0.5);   // Magenta
        vec3 b = vec3(0.2, 0.6, 0.8);   // Cyan
        vec3 c = vec3(0.9, 0.4, 0.1);   // Orange
        vec3 d = vec3(0.45, 0.75, 0.25); // Lime Green
    
        return a + b*cos( 6.28318*(c*t+d) );
    }
        out vec4 color;
        uniform float time;
        uniform uint viewport_width;
        uniform uint viewport_height;

       void main() {
            vec2 uv = (gl_FragCoord.xy * 2.0 - vec2(viewport_width,viewport_height)) / viewport_height;
            
            vec2 uv0 = uv;
            vec3 finalColor = vec3(0.0);
            
            for (float i = 0.0; i < 4.0; i++) {
                uv = fract(uv * 1.5) - 0.5;
        
                float d = length(uv) * exp(-length(uv0));
        
                vec3 col = palette(length(uv0) + i*.4 + time*.4);
        
                d = sin(d*8. + time)/8.;
                d = abs(d);
        
                d = pow(0.01 / d, 1.2);
        
                finalColor += col * d;
            }
                
            color = vec4(finalColor, 1.0);

       }
        
    
    "#;
    return fragment_shader;
}
