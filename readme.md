# Docker

A short summary for using Docker with this project.

## Basic setup
- A Dockerfile is provided to build the app image.
- Use docker-compose to run multiple services (app, db, etc.).

## Example Dockerfile (multi-stage)
```dockerfile
# build stage
FROM node:18-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# runtime stage
FROM node:18-alpine
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package*.json ./
RUN npm ci --production
CMD ["node", "dist/index.js"]
EXPOSE 3000
```

Adjust build/run steps for your stack (Python, Go, .NET, etc.).

## Example docker-compose.yml (dev)
```yaml
version: "3.8"
services:
    app:
        build: .
        ports:
            - "3000:3000"
        volumes:
            - .:/app
            - /app/node_modules
        environment:
            - NODE_ENV=development
        depends_on:
            - db

    db:
        image: postgres:15
        environment:
            POSTGRES_USER: app
            POSTGRES_PASSWORD: secret
            POSTGRES_DB: appdb
        volumes:
            - db-data:/var/lib/postgresql/data

volumes:
    db-data:
```

## Common commands
- Build and run: `docker-compose up --build`
- Run in background: `docker-compose up -d --build`
- Stop and remove containers: `docker-compose down`
- Build an image from Dockerfile: `docker build -t myapp:latest .`
- Run a container: `docker run --rm -p 3000:3000 myapp:latest`

## Notes
- Persist data with volumes (databases, uploads).
- Use a `.env` file for configuration and reference it in compose.
- For production: reduce image size (multi-stage), avoid mounting source, set NODE_ENV=production, and manage secrets securely.

If you want examples for a specific stack (e.g., Python/Django, .NET, Go), specify which one.