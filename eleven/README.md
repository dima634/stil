## Compiling protocol buffers
```bash
protoc -I=../proto --cpp_out=../eleven/src/eleven/proto ../proto/*
```

## Installing QML plugin
```bash
cmake --install /home/dima/repos/stil/eleven/out/build/(debug|release)
```
