// Copyright 2015 The Athena Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::{Path};
use glium::texture::{Texture2d};
use image;
use {Graphics};

pub struct Texture {
    texture: Texture2d
}

impl Texture {
    pub fn load(graphics: &Graphics, path: &str) -> Texture {
        let image = image::open(&Path::new(path)).unwrap();
        let texture = Texture2d::new(graphics.glium_display(), image);

        Texture {
            texture: texture
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.texture.get_width(), self.texture.get_height().unwrap())
    }

    pub fn glium_texture(&self) -> &Texture2d {
        &self.texture
    }
}