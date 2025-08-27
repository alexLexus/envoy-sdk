# libenvoy


## Ссылки на репозитории

- https://github.com/envoyproxy/envoy.git
- https://github.com/protocolbuffers/protobuf.git
- https://github.com/cncf/xds.git
- https://github.com/bufbuild/protoc-gen-validate.git
- https://github.com/googleapis/googleapis.git


## Как почистить .proto от лишних файлов

``` bash
for FILE in $(find proto -type f -not -name '*.proto' ) ; do
    rm $FILE
done

for DIR in $(find proto -type d -empty -delete) ; do
    rm -rf $DIR
done
```

## Пример сборки

https://github.com/hyperium/tonic/blob/master/examples/build.rs