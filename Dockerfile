# Stage 1: Frontend builder
FROM node:18-alpine as frontend-builder
WORKDIR /frontend
COPY ./frontend . 
RUN yarn install --frozen-lockfile
RUN yarn build

# Stage 2: Backend builder
FROM rust:bullseye as backend-builder
WORKDIR /app
COPY ./backend .
RUN cargo build --release

# Final stage: Combined image for serving backend and frontend
FROM debian:bullseye-slim
COPY --from=frontend-builder /frontend/build /usr/local/share/frontend_build
COPY --from=backend-builder /app/target/release/backend /usr/local/bin

ENV FRONTEND_PATH=/usr/local/share/frontend_build

EXPOSE 8080
CMD [ "backend" ]