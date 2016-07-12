# Docker Image Cleanup

At [Wongnai](https://www.wongnai.com), we build image from our CI tool. The
images are tagged with build number before pushing so we can track the running
version in the image field of Kubernetes.

However, this result in a lot of leftover images stored in the builder Docker
and so this project was born.

## Install

Make sure you have openssl installed

(or you can just copy from the release page)

```
cargo build --release
```

Put `target/release/imagecleanup` somewhere in your $PATH.

## Usage

Suppose you have the following images:

- test:1
- test:2
- test:3
- test:4
- test:5 test:latest

and you execute

```
imagecleanup --numbered test --numbered-keep 2
```

This will remove all images but

- test:3
- test:4
- test:5 test:latest (Images with multiple tags do not count)

Images within other repository are unaffected.

## License

(This project is done by me, out of work hours)

The MIT License (MIT)  
Copyright (c) 2016 Manatsawin Hanmongkolchai

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
