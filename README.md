# roman-api

# openapi generate
```shell
openapi-generator generate -i ./openapi.yaml -g typescript-axios -o ./client/src/generated
openapi-generator generate -i ./openapi.yaml -g rust-server -p packageName=generated -o ./server/generated
```