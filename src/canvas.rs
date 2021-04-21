use crate::tuples::Tuple;
use anyhow::Result;
use std::fmt::Write;

pub struct Canvas {
    pixels: Vec<Tuple>,
    height: usize,
    width: usize,
}

fn get_index(width: usize, x: usize, y: usize) -> usize {
    (y * width) + x
}

impl Canvas {
    pub fn new(height: usize, width: usize) -> Self {
        let pixels = vec![Tuple::new_colour(0.0, 0.0, 0.0); height * width];
        Canvas {
            pixels,
            height,
            width,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, colour: &Tuple) -> &mut Self {
        if let Some(pixel) = self.pixels.get_mut(get_index(self.width, x, y)) {
            *pixel = colour.clone()
        } else {
            panic!("Unknown index x:{} y:{}", x, y)
        }
        self
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Tuple> {
        self.pixels.get(get_index(self.width, x, y))
    }
}

fn clamp_color(max_colour: &usize, colour: &f32) -> usize {
    (*max_colour as f32 * colour)
        .min(*max_colour as f32)
        .max(0.0) as usize
}

pub fn canvas_to_ppm(canvas: &Canvas) -> Result<String> {
    let mut ppm = String::new();
    let max_colour = 255;
    write!(
        &mut ppm,
        "P3\n{} {}\n{}\n",
        canvas.width, canvas.height, max_colour
    )?;
    for line in canvas.pixels.chunks(5) {
        let str_line = line
            .iter()
            .map(|colour| {
                format!(
                    "{} {} {}",
                    clamp_color(&max_colour, &colour.r()),
                    clamp_color(&max_colour, &colour.b()),
                    clamp_color(&max_colour, &colour.g())
                )
            })
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(&mut ppm, "{}", str_line)?;
    }
    Ok(ppm)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_canvas() {
        let canvas = Canvas::new(10, 20);
        let black = Tuple::new_colour(0.0, 0.0, 0.0);
        assert_eq!(canvas.width, 20);
        assert_eq!(canvas.height, 10);
        assert_eq!(canvas.pixels.len(), 20 * 10);
        assert!(canvas.pixels.iter().all(|&c| c == black))
    }

    #[test]
    fn mutate_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let update_colour = Tuple::new_colour(1.0, 0.0, 0.0);
        canvas.set_pixel(5, 5, &update_colour);
        assert_eq!(canvas.get_pixel(5, 5).unwrap(), &update_colour);
    }
}
