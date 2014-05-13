// Copyright 2014 The sdl2-rs Developers.
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

// SDL_stdinc.h

#[deriving(Eq)]
#[repr(C)]
pub enum SDL_bool {
    SDL_FALSE = 0,
    SDL_TRUE = 1,
}

impl SDL_bool {
    #[inline]
    pub fn from_bool(x: bool) -> SDL_bool {
        match x {
            true => SDL_TRUE,
            false => SDL_FALSE,
        }
    }

    #[inline]
    pub fn to_bool(&self) -> bool {
        *self == SDL_TRUE
    }
}

pub type Sint8 = i8;
pub type Uint8 = u8;
pub type Sint16 = i16;
pub type Uint16 = u16;
pub type Sint32 = i32;
pub type Uint32 = u32;
pub type Sint64 = i64;
pub type Uint64 = u64;
