FROM debian:jessie
ADD target/release/imagecleanup /imagecleanup
ENTRYPOINT ["/imagecleanup"]
