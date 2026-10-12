# Third-party notices

Code ported into superfargoth from other projects, with their licences. Dependencies pulled through
Cargo carry their own licence files and are not listed here.

## bevy-aqua

- Source: https://github.com/sayhisam1/bevy-aqua, version 0.1.3, commit `98d755ff1ff13c509d8616bd41afeab403ae1409`.
- Licence: MIT OR Apache-2.0 (used here under MIT). The upstream `LICENSE-MIT` has no copyright line;
  the holder is recorded as "bevy-aqua contributors" (maintainer: sayhisam1).
- Ported into: `crates/sf_water/src/spectrum.rs` (JONSWAP peak and alpha relations, integer hash,
  Box-Muller pair), with the changes listed in that file's header.

```text
MIT License

Copyright (c) bevy-aqua contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
