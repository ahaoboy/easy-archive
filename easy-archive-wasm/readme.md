## install

```bash
cargo binstall easy-archive
```

```bash
pnpm install @easy-install/easy-archive -g
```

## NOTE

When reading the buffer field, the data in rust will be consumed. It needs to be
read as the last field, otherwise a null pointer exception will occur.

```ts
const { path, mode, isDir, buffer } = item
```
