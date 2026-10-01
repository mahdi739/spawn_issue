use std::time::Duration;

use freya::prelude::*;

fn main() {
  launch(LaunchConfig::new().with_window(WindowConfig::new(app)));
}

fn app() -> impl IntoElement {
  rect().expanded().center().child(
    Button::new()
      .on_press(move |_| {
        spawn_forever(async move {
          Platform::get().launch_window(WindowConfig::new(move || sub_app())).await;
        });
      })
      .child("New Window"),
  )
}

fn sub_app() -> impl IntoElement {
  let mut name = use_state(|| "hello".to_string());

  let on_press = move |_| {
    spawn_forever(async move {
      hold_str(name.read().cloned().as_str()).await;
      name.set("second".to_string());
    });
    Platform::get().close_window(Platform::window_id());
  };

  rect().expanded().center().child(Button::new().on_press(on_press).child("Click"))
}

async fn hold_str(s: &str) {
  timer(Duration::from_millis(50)).await;
  println!("still holding: {s}");
}
