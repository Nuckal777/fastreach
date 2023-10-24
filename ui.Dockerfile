FROM node:18-alpine as build
WORKDIR /app/fastreach-ui
COPY . /app
RUN npm install && npm run build

FROM caddy:2.7-alpine
COPY --from=build /app/fastreach-ui/dist /usr/share/caddy
