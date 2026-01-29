# Deploying MontRS Applications

MontRS applications are compiled to native binaries, making them easy to deploy across various environments, from traditional VPS to modern serverless platforms.

## 📦 Building for Production

Use the `build` command with the `--release` flag:

```bash
montrs build --release
```

This will produce a single, optimized binary in `target/release/`.

## 🐳 Docker Deployment

A typical `Dockerfile` for a MontRS app:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo install --path packages/cli
RUN montrs build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/my-app /usr/local/bin/my-app
CMD ["my-app"]
```

## ☁️ Cloud Platforms

### Railway / Render / Fly.io
These platforms can build your app using the `Dockerfile` above. Ensure you set the `PORT` and `DATABASE_URL` environment variables.

### AWS / GCP / Azure
Deploy the compiled binary to an EC2 instance, App Runner, or Cloud Run.

## ⚙️ Environment Configuration

MontRS uses `dotenv` support and environment variable validation via `montrs-core`. Ensure your production environment has the necessary variables defined:

```bash
DATABASE_URL=postgres://...
APP_SECRET=...
PORT=3000
```

## 🤖 Agents and Deployment

Agents can help generate CI/CD pipelines (e.g., GitHub Actions) by reading the `AppSpec` to understand the project's dependencies and build requirements.
