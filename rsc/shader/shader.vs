#version 140

in vec3 iPosition;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

// フラグメントシェーダーに変数を渡すために定義する
out vec3 FragPosition;

void main()
{
  // iPositionには頂点座標が入っている
  // uModelにはモデル行列が入っている
  // uViewにはビュー行列が入っている
  // uProjectionには射影行列が入っている

  // iPositionを４次元ベクトルに変換する
    FragPosition = vec3(uModel * vec4(iPosition, 1.0));
    // モデル行列にかける。ここでモデル行列が回転行列や縮小などであればここで実行される
    gl_Position = uProjection * uView * vec4(FragPosition, 1.0);
}