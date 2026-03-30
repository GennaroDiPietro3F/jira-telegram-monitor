FROM rust:latest

WORKDIR /app

COPY . .

RUN apt-get update && apt-get install -y \
    libssl3 \
    build-essential \
    pkg-config \
    libgtk-3-dev \
    libatk1.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libglib2.0-dev \
    libgdk-pixbuf-xlib-2.0-dev \
    libfontconfig1-dev \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libxcursor-dev \
    libxinerama-dev \
    libxfixes-dev \
    libxcomposite-dev \
    libxdamage-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

CMD ["./target/release/api_jira_for_tickets"]