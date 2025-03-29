use plotters::prelude::*;
use plotters::coord::combinators::IntoLogRange;

fn main() {    
    #[derive(Copy, Clone)]
    struct Punto {x: f64, y: f64}

    const N: u16 = 500;
    const ITERACIONES: u16 = 100;
    const RANGE: f64 = 2.0;
    let midpoint = Punto {x: -1.75, y: -0.35}; 
    let mut pasa: u64 = 0;
    let mut nopasa: u64 = 0;
 
    evcxr_figure((640, 480), |root| {
        _ = root.fill(&WHITE);

        let mut chart = ChartBuilder::on(&root)
            .caption("Uniform Distribution Scatter Plot", ("Arial", 20).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0f32..1f32, 0f32..1f32)?;

        chart.configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .y_labels(5)
            .x_label_formatter(&|x| format!("{:.1}", *x as f64 / 100.0))
            .y_label_formatter(&|y| format!("{}%", (*y * 100.0) as u32))
            .draw()?;

        let _ = chart.draw_series(random_samples.iter().map(|(x,y)| Circle::new((*x,*y), 3, GREEN.filled())));
        
        Ok(())
    }).style("width:100%");

    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let (xr, yr) = (chart.x_range(), chart.y_range());
 
    let mut p0;
    let mut c = Punto {x: 0.0, y: 0.0};
    let mut aux: u16;
    let mut image: [u16; 40002] = [0; 40002];
    let mut k: u16;
    let mut xtemp: f64;
    
    for i in 0..N {
		for j in 0..N {
			p0 = Punto {x:0.0, y:0.0};
			c.x = midpoint.x + 2.0 * RANGE * ((i as f64 / N as f64) - 0.5);
            c.y = midpoint.y + 2.0 * RANGE * ((j as f64 / N as f64) - 0.5);
            //println!("c.x, c.y {},{}__{}", c.x, c.y, (j as f64 / N as f64));

            aux = ITERACIONES;
            k = 0;
            while (p0.x*p0.x+p0.y*p0.y) < 4.0 && k < ITERACIONES {
                xtemp = p0.x*p0.x - p0.y*p0.x + c.x; 
                p0.y = f64::abs(2.0*p0.x*p0.y) + c.y; // abs returns the absolute value
                p0.x = xtemp;
                k = k + 1;
            }
            if k == ITERACIONES {
                pasa = pasa + 1;
            } else {
                nopasa = nopasa + 1;
            }
 
        
			//for k in 0..ITERACIONES {
				//p.x = p0.x*p0.x - p0.y*p0.y - c.x;
				//p.y = 2.0 * f64::abs(p0.x*p0.y) - c.y;
				//p0.x = p.x;
                //p0.y = p.y;
				//if p.x*p.x + p.y*p.y > 1.0 {
                    //aux = k;
					//break;
                //}
			//}
            //if aux == ITERACIONES {
                //pasa = pasa + 1;
            //} else {
                //nopasa = nopasa + 1;
            //}
            ////image[(j*N+i) as usize] = aux;
            //// println!("I {}", (j*N+i));
		}
	}
    
            //image[(j*N+i) as usize] = aux;
            // println!("I {}", (j*N+i));
            
    println!("Pasa {}", pasa);
    println!("noPasa {}", nopasa);
}
