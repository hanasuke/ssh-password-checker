from rust:1.41.0

WORKDIR /workdir
COPY . .

RUN cargo install --path .

CMD ["cargo", "build", "--release"]
