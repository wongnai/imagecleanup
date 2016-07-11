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
