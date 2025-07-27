FROM rustlang/rust:nightly AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/ouka /
COPY --from=builder /app/templates /templates
EXPOSE 3000
CMD ["/ouka"]
