FROM rust:1.73-slim-bookworm

RUN apt update && apt install -y zsh curl git vim \
    # To install Node.js
    ca-certificates gnupg

RUN rustup component add rustfmt

ARG NODEJS_VERSION=21
RUN mkdir -p /etc/apt/keyrings
RUN curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg
RUN echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_$NODEJS_VERSION.x nodistro main" | tee /etc/apt/sources.list.d/nodesource.list
RUN apt update && apt install -y nodejs
RUN npm install --global yarn

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended
