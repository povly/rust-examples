use std::time::{Duration, Instant};
fn pixel_loop<S>(mut state: S, update_fps: usize, update: fn(&mut S), render: fn(&mut S, dt: Duration)){
    if update_fps == 0 {
        panic!("Designated FPS for updates needts to be > 0");
    }
    let mut accum: Duration = Duration::new(0,0);
    let mut current_time = Instant::now();
    let mut last_time;

    let update_dt = Duration::from_nanos((1_000_000_000f64 / update_fps as f64).round() as u64);

    loop {
        last_time = current_time;
        current_time = Instant::now();
        let dt= current_time - last_time;

        while accum > update_dt {
            update(&mut state);
            accum -= update_dt;
        }

        render(&mut state, dt);

        accum += dt;
    }
}
#[derive(Default)]
struct State {
    updates_called: usize,
    renders_called: usize,
    time_passed: Duration
}

fn main() {
    let state = State::default();

    pixel_loop(state, 120, |s | {
        s.updates_called +=1;
    }, |s, dt | {
        s.renders_called +=1;
        s.time_passed += dt;
        if s.time_passed > Duration::from_secs(1) {
            println!("Update FPS: {:.2}", s.updates_called as f64 / 2f64);
            println!("Render FPS: {:.2}", s.renders_called as f64 / 2f64);

            s.updates_called = 0;
            s.renders_called = 0;
            s.time_passed = Duration::default();
        }

        std::thread::sleep(Duration::from_millis(16));
    })
}
