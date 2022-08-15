FROM rust

COPY src/assets/names-tab.dmp /tmp
COPY target/release/biotax /bin

ARG DATABASE_PATH=/tmp/names-tab.dmp
ENV DATABASE_PATH=${DATABASE_PATH}
ARG SERVICE_PORT=8080
ENV SERVICE_PORT=${SERVICE_PORT}

EXPOSE ${SERVICE_PORT}

CMD ["/bin/biotax"]
