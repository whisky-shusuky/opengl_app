use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // unwrap はエラー処理をサボっているので、実際にはエラー処理を書く
    let window = video_subsystem
        .window("SDL", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    // 色指定
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    // イベントループ
    // EventPump はイベントを取得するための構造体
    //
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // event_pump.poll_iter()で得られたイテレータをループで回してイベントキューをひとつづつ処理する
        for event in event_pump.poll_iter() {
            // rustはswitchではなくてmatch式で分岐させる
            match event {
                // ウィンドウの×ボタンが押されたらループを抜ける
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                // break 'running でrunningラベルのループを抜ける
                } => break 'running,
                _ => {}
            }
        }
    }

    // ループの中で何度も画面を白く塗りつぶす
    canvas.present();

    // 1/60秒待つ(60FPS)
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
