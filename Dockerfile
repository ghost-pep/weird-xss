FROM rust:latest
RUN apt update -y && apt install -y \ 
    openssl
COPY . .
RUN bash ./generate_ssl.sh
CMD ["cargo", "run"]
