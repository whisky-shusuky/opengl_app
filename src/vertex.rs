use std::mem;
use std::os::raw::c_void;

use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLsizeiptr};

pub struct Vertex {
    vao: u32,
    _vbo: u32,
    vertex_num: i32,
}

impl Vertex {
    // size: 頂点データのサイズ
    // data: 頂点データのポインタ
    // usage: どのようなアクセス頻度で頂点データを使うかを示す値
    // attribute_type_vec: 頂点属性の型を格納したベクター型
    // attribute_size_vec: 頂点属性のサイズを格納したベクター型
    // stride: 各頂点が何個おきに並んでいるかを示す値
    // vertex_num: 頂点数
    pub fn new(
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum,
        attribute_type_vec: std::vec::Vec<GLenum>,
        attribute_size_vec: std::vec::Vec<GLint>,
        stride: GLsizei,
        vertex_num: i32,
    ) -> Vertex {
        // vao,vboのidを格納する変数
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // create vertex array object and vertex buffer object
            // gpu上にvao,vboのメモリ確保(第一引数を増やせば複数一気のメモリ確保も可能)
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // bind buffer
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            //vbo に初めてデータを転送する場合は glBufferData を使用する(2回目以降は glBufferSubData を使用する)
            gl::BufferData(gl::ARRAY_BUFFER, size, data, usage);

            let mut offset = 0;
            for i in 0..attribute_type_vec.len() {
                // i番目の頂点属性を有効にする
                gl::EnableVertexAttribArray(i as u32);
                // i番目の頂点属性のデータのフォーマットを指定する
                // 　offsetがループごとに増えていくので、i番目の頂点属性のデータサイズ分だけずれる
                gl::VertexAttribPointer(
                    i as u32,                                              // 頂点属性の順番
                    attribute_size_vec[i],                                 // 頂点属性あたりの要素数
                    attribute_type_vec[i],                                 // 頂点属性の型
                    gl::FALSE,                                             // 正規化するかどうか
                    stride, // 頂点データの始まりが何個おきか
                    (offset * mem::size_of::<GLfloat>()) as *const c_void, // 頂点データの開始地点のオフセット
                );
                offset += attribute_size_vec[i] as usize;
            }

            // unbind
            // vao,vboのバインドを解除
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Vertex {
            vao: vao,
            _vbo: vbo,
            vertex_num: vertex_num,
        }
    }

    // 描画
    pub fn draw(&self) {
        unsafe {
            // vaoのバインド
            gl::BindVertexArray(self.vao);
            // 描画するプリミティブ、頂点データの開始地点インデックス、頂点数
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_num);
            // vaoのバインドを解除
            gl::BindVertexArray(0);
        }
    }
}
