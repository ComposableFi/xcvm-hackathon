FROM node:16.15.1

WORKDIR /usr/src/evm_contracts
COPY ./evm_contracts .
RUN npm install && npm run abis

FROM rust:1.61

WORKDIR /usr/src/relayer
COPY ./relayer .

WORKDIR /usr/src/relayer
RUN cargo install --path .

CMD ["relayer"]