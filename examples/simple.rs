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

extern crate jamkit;

fn main() {
    let mut display = jamkit::Graphics::init("test", 640, 480);
    let test_texture = jamkit::Texture::load(&display, "examples/test.png");

    let mut timer = jamkit::utils::DeterminismTimer::at_interval(10);
    let mut a_state = false;
    let mut d_state = false;
    let mut x = 0;

    'main: loop {
        for event in display.poll_events() {
            match event {
                jamkit::Event::Closed => break 'main,
                jamkit::Event::KeyboardInput(state, key) =>
                    if state.is_pressed() {
                        match key {
                            jamkit::Key::A => a_state = true,
                            jamkit::Key::D => d_state = true,
                            _ => {}
                        }
                    } else {
                        match key {
                            jamkit::Key::A => a_state = false,
                            jamkit::Key::D => d_state = false,
                            _ => {}
                        }
                    },
                _ => {}
            }
        }

        timer.update(&mut |_| {
            if a_state && !d_state { x -= 1; }
            if d_state && !a_state { x += 1; }
        });

        let mut frame = jamkit::Frame::start(&display);
        frame.draw(&test_texture, None, [0, 0, 200, 200]);
        frame.draw(&test_texture, Some([50, 50, 150, 150]), [x, 250, x + 100, 350]);
        frame.finish();
    }
}
