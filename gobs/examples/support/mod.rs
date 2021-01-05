#![allow(dead_code)]

use glium;
use glium::glutin::event::{Event, StartCause};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use std::time::{Duration, Instant};

pub mod camera;
pub mod main_loop;

pub enum Action {
    Stop,
    Continue,
}

pub fn identity_matrix() -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn scale_matrix(s: f32) -> [[f32; 4]; 4] {
    [
        [s, 0.0, 0.0, 0.0],
        [0.0, s, 0.0, 0.0],
        [0.0, 0.0, s, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn rotation_matrix(axis: [f32; 3], angle: f32) -> [[f32; 4]; 4] {
    let (s, c) = angle.sin_cos();
    let oc = 1.0 - c;

    [
        [
            oc * axis[0] * axis[0] + c,
            oc * axis[0] * axis[1] - axis[2] * s,
            oc * axis[2] * axis[0] + axis[1] * s,
            0.0,
        ],
        [
            oc * axis[0] * axis[1] + axis[2] * s,
            oc * axis[1] * axis[1] + c,
            oc * axis[1] * axis[2] - axis[0] * s,
            0.0,
        ],
        [
            oc * axis[2] * axis[0] - axis[1] * s,
            oc * axis[1] * axis[2] + axis[0] * s,
            oc * axis[2] * axis[2] + c,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn multiply_matrix(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = identity_matrix();

    for i in 0..=3 {
        for j in 0..=3 {
            result[j][i] = 0.0;

            for k in 0..=3 {
                result[j][i] += a[k][i] * b[j][k];
            }
        }
    }
    result
}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F) -> !
where
    F: 'static + FnMut(&Vec<Event<()>>) -> Action,
{
    let mut events_buffer = Vec::new();
    let mut next_frame_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        let run_callback = match event.to_static() {
            Some(Event::NewEvents(cause)) => match cause {
                StartCause::ResumeTimeReached { .. } | StartCause::Init => true,
                _ => false,
            },
            Some(event) => {
                events_buffer.push(event);
                false
            }
            None => {
                // Ignore this event.
                false
            }
        };

        let action = if run_callback {
            let action = callback(&events_buffer);
            next_frame_time = Instant::now() + Duration::from_nanos(16666667);
            // TODO: Add back the old accumulator loop in some way

            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                *control_flow = ControlFlow::WaitUntil(next_frame_time);
            }
            Action::Stop => *control_flow = ControlFlow::Exit,
        }
    })
}
