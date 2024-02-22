FROM rust:1.76 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update
RUN apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu

WORKDIR /app

# download dependencies first
COPY Cargo.lock Cargo.toml ./

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

# build phantom app first for caching dependency
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo fetch
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm -rf src

# build real app
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rush ./
COPY --from=builder /app/.env.docker ./.env

CMD ["/app/rush", "--db-password", ""]