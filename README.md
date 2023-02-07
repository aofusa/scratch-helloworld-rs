Scratch Hello World
=====

scratchコンテナで動作するHello WorldをRustで作成する

no_std環境でシステムコールを直接呼び出してみる

- ビルドと実行
```sh
docker build -t dev.local/hello-world:latest .

docker run dev.local/hello-world
```

参考
[hello-worldコンテナ](https://hub.docker.com/_/hello-world])
[Rust でしっかりとスタティックリンク](https://qiita.com/moriai/items/b1fa7d1b43d985d408cc)
