FROM rust:1

RUN cargo -V

COPY . .

RUN cargo build --release

CMD ["./target/release/pagefunction"]
