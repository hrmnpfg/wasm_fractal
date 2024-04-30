mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Pixel
{
    r: u8,
    g: u8,
    b: u8,
}

#[wasm_bindgen]
pub struct Image
{
    height: usize,
    width: usize,
    contents: Vec<Pixel>,
    sx: f64,
    sy: f64,
    ex: f64,
    ey: f64,
    count: i32
}

#[wasm_bindgen]
impl Pixel
{
    pub fn new() -> Pixel
    {
        Pixel { r: 0, g: 0, b: 0,}
    }
    pub fn set_color(&mut self, r: u8, g: u8, b: u8)
    {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}

impl Pixel
{
    pub fn get_color(&self) -> (u8,u8,u8)
    {
        (self.r,self.g,self.b)
    }
}

#[wasm_bindgen]
impl Image
{
    pub fn new() -> Image
    {
        let h = 512;
        let w = 512;
        let target = (-0.7756837680090538,0.13646736829469008);
        let size = 2.0;
        let mut img = Image { height: h, width: w, contents: vec!(Pixel::new();h*w), sx: target.0-size, sy: target.1-size, ex: target.0+size, ey: target.1+size, count: 40};
        img.fill();
        img
    }

    pub fn tick(&mut self)
    {
        let step_zoom_x = (self.ex-self.sx)/8.0;
        let step_zoom_y = (self.ey-self.sy)/8.0;
        self.sx+=step_zoom_x;
        self.sy+=step_zoom_y;
        self.ex-=step_zoom_x;
        self.ey-=step_zoom_y;
        self.fill();
        self.count += 5;
    }

    pub fn get_height(&self) -> u32
    {
        self.height as u32
    }
    pub fn get_width(&self) -> u32
    {
        self.width as u32
    }
    pub fn contents(&self) -> *const Pixel
    {
        self.contents.as_ptr()
    }

}

impl Image
{
    pub fn fill(&mut self)
    {
        let max_counter=self.count;
        let step_x: f64 = (self.ex - self.sx)/self.width as f64;
        let step_y: f64 = (self.ey - self.sy)/self.height as f64;
        for (i,px) in self.contents.iter_mut().enumerate()
        {
            let x = self.sx + step_x * (i%self.width) as f64;
            let y = self.sy + step_y * (i/self.width) as f64;
            let mut counter: i32 = 0;
            let cr=x;
            let ci=y;
            let mut zr=0.0;
            let mut zi=0.0;
            while zr*zr+zi*zi <= 4.0 && counter < max_counter
            {
                let xtemp = zr*zr-zi*zi+cr;
                zi = 2.0*zi*zr+ci;
                zr = xtemp;
                counter+=1;
            }
            if counter<max_counter
            {
                //px.set_color(100*counter as u8,(128.0*zr) as u8,(128.0*zi) as u8);
                px.set_color((counter%64*4) as u8,(counter%8*10) as u8,(counter%6*10) as u8);
            }
            else
            {
                //px.set_color((8.0*zr) as u8,(8.0*zi) as u8,((zr*zr+zi*zi)*128.0) as u8);
                px.set_color(0,0,0 as u8);
            }
        }
    }
}
