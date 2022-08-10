FROM rust

COPY src/assets/names-tab.dmp /tmp
COPY target/release/biotax /bin

ENV DATABASE_PATH=/tmp/names-tab.dmp

EXPOSE 8080

CMD ["/bin/biotax"]
