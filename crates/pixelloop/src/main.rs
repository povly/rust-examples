use std::num::NonZeroUsize;
use std::time::{Duration, Instant};

/// Максимальный кусок реального времени, который цикл готов принять за один кадр.
/// Защита от «спирали смерти»: зависший кадр не должен рождать сотни update подряд.
const MAX_DT: Duration = Duration::from_millis(250);

fn pixel_loop<S>(
    mut state: S,
    update_fps: NonZeroUsize,
    render_fps: NonZeroUsize,
    mut update: impl FnMut(&mut S),
    mut render: impl FnMut(&mut S, Duration) -> bool,
) {
    let update_dt = Duration::from_secs_f64(1.0 / update_fps.get() as f64);
    let frame_dt = Duration::from_secs_f64(1.0 / render_fps.get() as f64);

    let mut accum = Duration::ZERO;
    let mut last_time = Instant::now();

    loop {
        let frame_start = Instant::now();
        let dt = (frame_start - last_time).min(MAX_DT);
        last_time = frame_start;

        accum += dt;
        while accum >= update_dt {
            update(&mut state);
            accum -= update_dt;
        }

        if !render(&mut state, dt) {
            break;
        }

        let frame_end = frame_start + frame_dt;
        let now = Instant::now();
        if frame_end > now {
            std::thread::sleep(frame_end - now);
        }
    }
}
#[derive(Default)]
struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration,
    reports_done: usize,
}

fn main() {
    let state = State::default();
    let update_fps = NonZeroUsize::new(120).expect("update fps");
    let render_fps = NonZeroUsize::new(60).expect("render fps");

    pixel_loop(
        state,
        update_fps,
        render_fps,
        |s| {
            s.updates_called += 1;
        },
        |s, dt| {
            s.renders_called += 1;
            s.time_passed += dt;

            if s.time_passed > Duration::from_secs(1) {
                let secs = s.time_passed.as_secs_f64();
                println!("Update FPS: {:.2}", s.updates_called as f64 / secs);
                println!("Render FPS: {:.2}", s.renders_called as f64 / secs);
                println!();

                s.updates_called = 0;
                s.renders_called = 0;
                s.time_passed = Duration::ZERO;

                s.reports_done += 1;
                return s.reports_done < 3;
            }

            true
        },
    );
}
