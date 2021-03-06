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

use ffi::stdinc::{SDL_bool, Uint32, Uint64};
use libc::{c_int, c_void};

// SDL_timer.h

pub type SDL_TimerCallback = extern "C" fn(interval: Uint32, param: *c_void) -> Uint32;

pub type SDL_TimerID = c_int;

extern "C" {
    pub fn SDL_GetTicks() -> Uint32;
    pub fn SDL_GetPerformanceCounter() -> Uint64;
    pub fn SDL_GetPerformanceFrequency() -> Uint64;
    pub fn SDL_Delay(ms: Uint32);
    pub fn SDL_AddTimer(interval: Uint32, callback: SDL_TimerCallback, param: *c_void) -> SDL_TimerID;
    pub fn SDL_RemoveTimer(id: SDL_TimerID) -> SDL_bool;
}
