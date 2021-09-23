# Descartes

`euclid`-compatible grid data structure.

(`euclid` is Servo's geometry library, so supporting its API seems like a good idea!)

## Specifics

This is basically a vendored version of the `gridd` library with:

- a few of the more unusual utility methods removed
- support for `euclid` data types
- iterator support for `euclid` rects

## License

Dual Apache/MIT, same as Rust!

## Links

Library links:

- https://github.com/FroshVII/gridd
- https://github.com/servo/euclid

## FOSS compliance

To comply with the Apache license for `gridd`, I've included Apache-compliant attribution notices in every file modified from `gridd/src/lib.rs` (the only file I borrowed).

To comply with the MIT license, here's the MIT notice from Gridd:

> Copyright (c) 2019 Chadbourne M.B. Smith
>
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
>
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.