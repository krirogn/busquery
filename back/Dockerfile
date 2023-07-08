# Build step
FROM rust:1.70 as build
WORKDIR /usr/src/busquery-api
COPY . .
RUN cargo install --path .

# Run step
FROM gcr.io/distroless/cc-debian10
COPY --from=build /usr/local/cargo/bin/busquery-api /usr/local/bin/busquery-api
CMD ["busquery-api"]