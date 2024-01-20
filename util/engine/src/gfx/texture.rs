use std::io::Cursor;

use glium::{Texture2d, texture::RawImage2d};
use ahash::AHashMap;
use rustc_hash::FxHashMap;
use crate::{GfxError, Display};
use image::{ImageBuffer, Rgb, ImageOutputFormat};

/// Manages textures.
pub struct Textures {
  textures: AHashMap<String, TextureInfo>,
  samplers: FxHashMap<u16, Texture2d>,
  next_sampler_id: u16,
}

impl Textures {
  /// Create a new texture manager.
  pub fn new(display: &Display) -> Result<Self, GfxError> {
    // Create the texture manager.
    let mut textures = Self {
      textures: AHashMap::new(),
      samplers: FxHashMap::default(),
      next_sampler_id: 0,
    };
    // Add a blank sampler.
    let image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(1, 1, Rgb([255, 255, 255]));
    let mut bytes = Vec::new();
    let mut cursor = Cursor::new(&mut bytes);
    image.write_to(&mut cursor, ImageOutputFormat::Png)?;
    textures.add_sampler(
      display, 
      bytes, 
      [(
        String::default(), 
        vec![
          [0.0, 0.0],
          [1.0, 0.0],
          [1.0, 1.0],
          [0.0, 1.0],
        ],
      )],
    )?;
    // Return the texture manager.
    Ok(textures)
  }
  /// Get a texture's information.
  #[inline]
  pub fn get_texture_info(&self, texture: &String) -> Result<&TextureInfo, GfxError> {
    self.textures.get(texture).ok_or_else(|| GfxError::TextureNotFound(texture.clone()))
  }
  /// Add a new sampler.
  pub fn add_sampler(
    &mut self, 
    display: &Display,
    bytes: impl AsRef<[u8]>, 
    info: impl IntoIterator<Item = (impl ToString, Vec<[f32; 2]>)>,
  ) -> Result<(), GfxError> {
    // Generate a sampler id.
    let sampler_id = self.next_sampler_id;
    self.next_sampler_id += 1;
    // Create the sampler. Note that this should be done prior to adding
    // the texture information, since if the sampler cannot be created, there
    // should not be textures added.
    let sampler = {
      let image = image::load_from_memory(bytes.as_ref())?.to_rgba8();
      let dimensions = image.dimensions();
      let raw = RawImage2d::from_raw_rgba(image.into_raw(), dimensions);
      Texture2d::new(display, raw)?
    };
    // Add the sampler.
    self.samplers.insert(sampler_id, sampler);
    // Loop through info and add the textures.
    for (texture, texture_coords) in info.into_iter() {
      let texture_info = TextureInfo {
        sampler_id: sampler_id,
        texture_coords: texture_coords.into_boxed_slice(),
      };
      self.textures.insert(texture.to_string(), texture_info);
    }
    Ok(())
  }
  /// Get a sampler from it's id.
  #[inline]
  pub fn get_sampler(&self, id: u16) -> Result<&Texture2d, GfxError> {
    self.samplers.get(&id).ok_or(GfxError::SamplerNotFound(id))
  }
}

/// The information about a texture.
pub struct TextureInfo {
  pub sampler_id: u16,
  pub texture_coords: Box<[[f32; 2]]>,
}