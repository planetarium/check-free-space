# @planetarium/check-free-space

This package uses native OS APIs to get free size available on specified path. It is powered by [`fs2`](https://github.com/danburkert/fs2-rs) under the hood.

## APIs

### getFreeSpace

```typescript
import { getFreeSpace } from '@planetarium/check-free-space';

getFreeSpace('./')
```

Gets free space available on specified path. **It returns BigInt.**

### checkFreeSpace

```typescript
import { checkFreeSpace } from '@planetarium/check-free-space';

if (checkFreeSpace('./', BigInt(1_000_000_000_000))) {
  console.log('Enough space available');
}
```

Checks if there are enough space to store specified amount of bytes in a specified path. You can provide a BigInt, string, or number value for bytes.

If you passed a string as bytes, it may throw if the string cannot be parsed as numerical value. If BigInt is passed, it will be converted to Rust `u64` during the process.