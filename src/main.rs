use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // init OpenGL
    // バージョン指定などしている
    // {}で囲っているのはプロファイルの指定が終わったらgl_attrが自動的に破棄されるようにするため
    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    // OpenGLバージョンのAPIを読み込む
    let _gl_context = window.gl_create_context().unwrap();
    // gl::load_with : 引数として関数を要求する。(文字列を引数にして関数ポインタを返す関数)
    // video_subsystem.gl_get_proc_address を引数にしてAPIの関数ポインタを受け取っている
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    // vertex shader, fragment shaderのパスを読み込みコンパイルする
    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    // set buffer
    // 頂点情報を格納する配列
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_LEN] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    // vertex.rsで定義したVertexを使って頂点情報をセットする
    let vertex = Vertex::new(
        (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
        buffer_array.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        vec![gl::FLOAT],
        vec![FLOAT_NUM as i32],
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei,
        VERTEX_NUM as i32,
    );

    //
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // openGLの描画処理はunsefeで囲む(C言語由来)
        unsafe {
            // 描画場所指定(ここでは画面全体)
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // clear screen
            // 背景色を指定
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            // 描画する際に利用しているカラーバッファーを初期化する
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // init matrice for model, view and projection
            // モデル行列（回転。縮小などしたい時に使う、何もしなければ単位行列）
            // こんなふうにすればz軸基準の回転行列が作れる
            // letmodel_matrix=Matrix4::from_angle_z(cgmath::Rad(f32::consts::PI));
            let model_matrix = Matrix4::identity();
            // ビュー行列（カメラの位置などを指定する）
            // ここではカメラの位置が(0,0,5)で、カメラが(0,0,0)を見ているということを指定して,上方向がy軸になるようにしている
            let view_matrix = Matrix4::look_at(
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                },
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            );
            // 透視投影法の行列を作成
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // shader use matrices
            // 上で定義したvertex shaderとfragment shaderを使う
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);

            // windowに描画する
            vertex.draw();

            // SDLのもので、画面に描画したものを更新する
            window.gl_swap_window();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
