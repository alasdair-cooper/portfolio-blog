FROM rust:1.88-bookworm as builder

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall trunk
RUN rustup target add wasm32-unknown-unknown

COPY . /

RUN trunk build --release

FROM nginx:latest

COPY --from=builder /dist /usr/share/nginx/html

CMD ["nginx", "-g", "daemon off;"]