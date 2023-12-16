FROM node:18-alpine as ui-build
WORKDIR /app/fastreach-ui
COPY . /app
RUN npm install && npm run build

FROM caddy:2.7-builder-alpine AS caddy-build
RUN xcaddy build --with github.com/RussellLuo/caddy-ext/ratelimit

FROM caddy:2.7-alpine
COPY --from=caddy-build /usr/bin/caddy /usr/bin/caddy
COPY --from=ui-build /app/fastreach-ui/dist /usr/share/caddy
