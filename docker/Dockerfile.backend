
FROM debian:sid

RUN rm -f /etc/apt/sources.list.d/*
RUN echo 'deb http://mirrors.ustc.edu.cn/debian sid main' > /etc/apt/sources.list
RUN apt update
RUN apt install -y libsqlite3-0 libssl3 libpq5
RUN apt clean

RUN mkdir /opt/pengzu/
COPY bin/backend .env /opt/pengzu/

EXPOSE 3000
WORKDIR /opt/pengzu
CMD ["/opt/pengzu/backend"]

