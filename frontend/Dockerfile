# -----------------------------
# 1. Build stage
# -----------------------------
FROM node:20 AS build

WORKDIR /app
COPY package*.json ./

RUN npm install
COPY . .

RUN npm run build

# -----------------------------
# 2. Nginx static server
# -----------------------------
FROM nginx:stable

COPY --from=build /app/dist /usr/share/nginx/html

EXPOSE 5173 80

CMD ["nginx", "-g", "daemon off;"]
