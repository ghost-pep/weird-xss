FROM rust:latest
RUN apt update -y && apt install -y \ 
    openssl
COPY . .
RUN /generate_ssl.sh
CMD ["cargo", "run"]
