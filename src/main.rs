use macroquad::prelude::*;

#[macroquad::main("Burning Ship")]
async fn main() {    
    #[derive(Copy, Clone)]
    struct Punto {x: f64, y: f64}

    const N: u16 = 500;
    const ITERACIONES: u16 = 100;
    const RANGE: f64 = 0.25;

    let mut escala: f64 = RANGE;

    let midpoint = Punto {x: -1.50, y: -0.0}; 
    let mut p0: Punto;
    let mut c: Punto;
    let mut k: u16;
    let mut xtemp: f64;

    let mut image = Image::gen_image_color(N as u16, N as u16, WHITE);
    let mut texture = Texture2D::from_image(&image);
    let mut fuerza_refresco = true;
    
    loop {
        let (_mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
        if 0.0 != mouse_wheel_y || fuerza_refresco {
            fuerza_refresco = false;

            if mouse_wheel_y == -1.0 {
                escala = escala + (escala / 5.0);
            } 
            if mouse_wheel_y == 1.0 {
                escala = escala - (escala / 5.0);
            }

            image = Image::gen_image_color(N as u16, N as u16, WHITE);
            texture = Texture2D::from_image(&image);

            c = Punto {x: 0.0, y: 0.0};
            for i in 0..N {
                for j in 0..N {
                    p0 = Punto {x:0.0, y:0.0};
                    c.x = midpoint.x + 2.0 * escala * ((i as f64 / N as f64) - 0.5);
                    c.y = midpoint.y + 2.0 * escala * ((j as f64 / N as f64) - 0.5);
                    
                    k = 0;
                    while (p0.x*p0.x+p0.y*p0.y) < 4.0 && k < ITERACIONES {
                        xtemp = p0.x*p0.x - p0.y*p0.x + c.x; 
                        p0.y = f64::abs(2.0*p0.x*p0.y) + c.y;
                        p0.x = xtemp;
                        k = k + 1;
                    }
                    if k == ITERACIONES {
                        image.set_pixel(i as u32, j as u32, BLACK);
                    } else {
                        image.set_pixel(i as u32, j as u32, Color::new(1.0 - k as f32/ITERACIONES as f32, 1.0 - k as f32/ITERACIONES as f32, k as f32/ITERACIONES as f32, 1.0 as f32));
                    }
                }
            }
        }
        let texto = format!("{}", escala);
        draw_text(&texto, 20.0, 20.0, 30.0, RED);

        texture.update(&image);
        draw_texture_ex(&texture, 0., 0., WHITE,  DrawTextureParams {dest_size: Some(vec2(screen_width(), screen_height())), ..Default::default()});
        next_frame().await;
    }
}
