# --- Build Stage ---
FROM rust:bookworm AS builder

WORKDIR /app

# 複製相依性設定檔
COPY Cargo.toml Cargo.lock ./

# 建立暫時性原始碼以快取已編譯的相依套件
RUN mkdir -p src && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 複製實際原始碼
COPY src ./src

# 更新時間戳記確保 Cargo 重新編譯專案本身
RUN touch src/lib.rs src/main.rs && \
    cargo build --release

# --- Runtime Stage ---
FROM debian:bookworm-slim AS runner

WORKDIR /app

# 安裝連線所需的 CA 憑證、OpenSSL 動態函式庫與時區資料
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      ca-certificates \
      libssl3 \
      tzdata && \
      rm -rf /var/lib/apt/lists/*

# 建立非 root 使用者提升安全性
RUN useradd -m -u 10001 -s /bin/sh botuser
USER botuser

# 複製編譯完成的執行檔
COPY --from=builder /app/target/release/twdash-bot /app/twdash-bot

# 預設環境變數
ENV RUST_LOG=info \
    TZ=Asia/Taipei

ENTRYPOINT ["/app/twdash-bot"]
