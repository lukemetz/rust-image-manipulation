extern crate png;
use png::{load_png, store_png};
use std::num::{from_u8, from_u32};
use std::rand::random;
//TODO see numpy / opencv image.
//resize
//better constructors
//Somedrawing methods
//Make a trait
//immutable tree diff type structure for image
//

pub struct Image {
  pub width: uint,
  pub height: uint,
  pub color_type: png::ColorType,
  pub data: Vec<(f32,f32,f32,f32)>
}

impl std::fmt::Show for Image {
  fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::FormatError> {
    write!(f, "Image\\{width:{}, height:{}\\}", self.width, self.height)
  }
}

impl Image {
  pub fn new_from_libpng(pngimg : Box<png::Image>) -> Image {
    println!("{},{}", (pngimg.width, pngimg.height), pngimg.pixels.len());
    if pngimg.color_type != png::RGBA8 {
      fail!("Only supports RGBA8");
    }
    let num_pixel = (pngimg.width * pngimg.height) as uint;

    let p = &pngimg.pixels;
    let mut data : Vec<(f32, f32, f32, f32)> = Vec::with_capacity(num_pixel);
    for i in range(0u, num_pixel as uint) {
      let color = (from_u8::<f32>(*p.get(i*4+0)).unwrap()/ 255.,
                   from_u8::<f32>(*p.get(i*4+1)).unwrap()/ 255.,
                   from_u8::<f32>(*p.get(i*4+2)).unwrap()/ 255.,
                   from_u8::<f32>(*p.get(i*4+3)).unwrap()/ 255.);
      data.push(color);
    }

    Image {
      width : from_u32(pngimg.width).unwrap(),
      height : from_u32(pngimg.height).unwrap(),
      color_type : pngimg.color_type,
      data : data
    }
  }

  pub fn get<'a>(&'a self, x : uint, y : uint) -> &'a(f32, f32, f32, f32) {
    self.data.get(y*self.width+x)
  }

  pub fn get_mut<'a>(&'a mut self, x : uint, y : uint) -> &'a mut (f32, f32, f32, f32) {
    self.data.get_mut(y*self.width+x)
  }

  pub fn to_libpng(&self) -> png::Image {
    let num_pixel = self.width * self.height;
    let mut pixels : Vec<u8> = Vec::with_capacity(num_pixel * 4);
    for j in range(0u, num_pixel) {
      let &(r, g, b, a) = self.data.get(j);
      pixels.push((255. * r).to_u8().unwrap());
      pixels.push((255. * g).to_u8().unwrap());
      pixels.push((255. * b).to_u8().unwrap());
      pixels.push((255. * a).to_u8().unwrap());
    }

    png::Image {
      width : self.width.to_u32().unwrap(),
      height : self.height.to_u32().unwrap(),
      color_type : self.color_type,
      pixels : pixels
    }
  }


  //Fun methods
  pub fn add_rectangle(&mut self, start : (uint, uint), end : (uint, uint),
    op : |uint, uint| -> (f32, f32, f32, f32)) {

    for x in range(start.val0(), end.val0()) {
      for y in range(start.val1(), end.val1()) {
        *self.get_mut(x,y) = op(x,y);
      }
    }
  }
}

pub fn get_color_rect(img : &Image, start : (uint, uint), end : (uint, uint)) -> (f32, f32, f32, f32) {
  let mut accum = (0., 0., 0., 0.);
  for x in range(start.val0(), end.val0()) {
    for y in range(start.val1(), end.val1()) {
      let &(r,g,b,a) = img.get(x,y);
      *accum.mut0() += r;
      *accum.mut1() += g;
      *accum.mut2() += b;
      *accum.mut3() += a;
    }
  }
  let num_pixel = (end.val0() - start.val0()) * (end.val1() - start.val1());
  let div = num_pixel.to_f32().unwrap();
  *accum.mut0() /= div;
  *accum.mut1() /= div;
  *accum.mut2() /= div;
  *accum.mut3() /= div;
  accum
}

pub fn main() {
  let img = match load_png(&Path::new("img.png")) {
    Ok(img) => box img,
    Err(string) => fail!("{}", string)
  };
  let mut img = Image::new_from_libpng(img);

  let pixels_per_slice = 15u;
  let num_slice = img.width/pixels_per_slice;
  let height = img.height.to_f32().unwrap();
  for x in range(0, num_slice) {
    let num_split = random::<uint>() % 10 + 10;
    let mut randoms_f = Vec::from_fn(num_split, |_| random::<f32>());
    randoms_f.push(0.);
    randoms_f.push(1.);
    let mut randoms = Vec::from_fn(randoms_f.len(), |x| (randoms_f.get(x)*height).to_uint().unwrap());

    randoms.sort();

    for y in range(0u, num_split+1) {
      let start = (x * pixels_per_slice, *randoms.get(y));
      let end = ((x+1) * pixels_per_slice, *randoms.get(y+1));

      let color = get_color_rect(&img, start, end);
      img.add_rectangle(start, end, |_,_| (color));
    }
  }

  println!("{}", img);
  let back = img.to_libpng();
  let _ = store_png(&back, &Path::new("back.png"));
}
