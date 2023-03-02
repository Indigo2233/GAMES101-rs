动态链接了Clib里包装为C库的C++库.

动态库的编译：`g++ -shared -o libobjloader.so OBJ_Loader_C.cpp -lc -fPIC`, 已经在`build.rs`中给出.

运行时需要配置环境变量：`export LD_LIBRARY_PATH=./Clib`.

运行方式：
```bash
export LD_LIBRARY_PATH=./Clib
cargo run -- output.png displacement
```

`displacement`可更换为其他shader.