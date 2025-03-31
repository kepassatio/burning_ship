use macroquad::prelude::*;

#[macroquad::main("Burning Ship")]
async fn main() {    
    #[derive(Copy, Clone)]
    struct Punto {x: f64, y: f64}

    const N: u16 = 500;
    const ITERACIONES: u16 = 1000;
    const RANGE: f64 = 0.25;

    let mut escala: f64 = RANGE;

    let mut midpoint = Punto {x: -1.75, y: -0.0}; 
    let mut p0: Punto;
    let mut c: Punto;
    let mut k: u16;
    let mut xtemp: f64;

    let mut image = Image::gen_image_color(N as u16, N as u16, WHITE);
    let mut texture = Texture2D::from_image(&image);
    let mut mouse_pos_x:f32;
    let mut mouse_pos_y:f32;
    let mut fuerza_refresco = true;

    (mouse_pos_x, mouse_pos_y) = mouse_position();
    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (aux_x, aux_y) = mouse_position();
            let escala_aux = escala / 5.0;
            if aux_x > mouse_pos_x {
                midpoint.x = midpoint.x - escala_aux;
            } else if aux_x < mouse_pos_x {
                midpoint.x = midpoint.x + escala_aux;
            }
            if aux_y > mouse_pos_y {
                midpoint.y = midpoint.y - escala_aux;
            } else if aux_y < mouse_pos_y {
                midpoint.y = midpoint.y + escala_aux;
            }
            mouse_pos_x = aux_x;
            mouse_pos_y = aux_y;
            fuerza_refresco = true;

        }

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
                        
                        let normalizado:f32 = k as f32 / ITERACIONES as f32 ;
                        let mut color_red: f32  = 0.0;
                        let mut color_green: f32 = 0.0;
                        let mut color_blue: f32 = 0.0;
                        match normalizado {
                            0.0..0.80 => color_red = normalizado,
                            0.80..0.95 => color_green = normalizado,
                            _ => color_blue = normalizado,
                        }
                        image.set_pixel(i as u32, j as u32, Color::new(color_red, color_green, color_blue, 1.0 as f32));
                    }
                }
            }
        }
        let texto = format!("{} {}", "Escala", escala);
        draw_text(&texto, 0.0, 15.0, 25.0, RED);

        texture.update(&image);
        draw_texture_ex(&texture, 0., 25.0, WHITE,  DrawTextureParams {dest_size: Some(vec2(screen_width(), screen_height()-25.0)), ..Default::default()});
        next_frame().await;
    }
}
