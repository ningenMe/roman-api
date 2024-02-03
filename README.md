# roman-api

# openapi generate
```shell
openapi-generator generate -i ./openapi.yaml -g typescript-axios -o ./client/src/generated
openapi-generator generate -i ./openapi.yaml -g rust-server -p packageName=generated -o ./server/generated
```

```shell
curl -X POST -d '{"bookmarkDirectoryList":[{"title":"test1","bookmarkList":[{"title":"a","url":"b"},{"title":"c","url":"d"}]}]}' http://127.0.0.1:50051/bookmarks
curl -X GET http://127.0.0.1:50051/bookmarks
```