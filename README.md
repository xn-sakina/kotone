# kotone

Transform file to UTF-8 by Rust.

### Install

```bash
  pnpm i -D kotone
```

### Usage

```ts
import fs from 'fs'

import { transform } from 'kotone' // or import `transformSync`

const buffer = fs.readFileSync('file.txt')
const utf8 = await transform(buffer)
fs.writeFileSync("file-utf8.txt", utf8, 'utf-8')
```

### Thanks/Inspiration

 - [sjis-utf8](https://github.com/jamesgordo/sjis-utf8): A small Node CLI for converting Shift-JIS encoded file into a UTF-8 encoded file.

### License

MIT
