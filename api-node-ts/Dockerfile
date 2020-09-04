FROM node:14.9.0-buster-slim

RUN apt-get update && apt-get install -y \
    tini \
    && rm -rf /var/lib/apt/lists/*

RUN useradd svc
WORKDIR /app

COPY --chown=svc:svc *.json ./
RUN npm install

COPY --chown=svc:svc src src
RUN npm run build

USER svc

ENTRYPOINT ["/usr/bin/tini", "--"]

EXPOSE 8000
CMD ["node", "dist/server.js"]
