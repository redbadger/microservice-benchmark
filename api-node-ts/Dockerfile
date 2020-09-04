FROM node:14.9.0-buster-slim

RUN apt-get update && apt-get install -y \
    tini \
    && rm -rf /var/lib/apt/lists/*

RUN useradd svc
RUN mkdir -p /app
RUN chown svc:svc /app

COPY *.json /app/
RUN cd /app && npm install

COPY src /app/src
RUN cd /app && npm run build

USER svc

RUN ls -l /app

ENTRYPOINT ["/usr/bin/tini", "--"]

EXPOSE 8000
CMD ["node", "/app/dist/server.js"]