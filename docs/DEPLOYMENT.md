# Deployment Guide

This guide covers various deployment strategies for the Rust URL Shortener.

## Table of Contents

1. [Local Development](#local-development)
2. [Docker Deployment](#docker-deployment)
3. [Cloud Deployment](#cloud-deployment)
4. [Production Considerations](#production-considerations)

---

## Local Development

### Prerequisites

- Rust 1.56 or higher
- SQLite 3
- Diesel CLI

### Setup

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd rust-url-shortener
   ```

2. **Install Diesel CLI:**
   ```bash
   cargo install diesel_cli --no-default-features --features sqlite
   ```

3. **Create environment file:**
   ```bash
   cp .env.example .env
   ```

4. **Run database migrations:**
   ```bash
   diesel migration run
   ```

5. **Build and run:**
   ```bash
   cargo run
   ```

The server will be available at `http://localhost:8080`

---

## Docker Deployment

### Using Docker

1. **Build the image:**
   ```bash
   docker build -t rust-url-shortener .
   ```

2. **Run the container:**
   ```bash
   docker run -d \
     -p 8080:8080 \
     -e DATABASE_URL=rust_url_shortener.db \
     -e BASE_URL=http://localhost:8080 \
     --name url-shortener \
     rust-url-shortener
   ```

3. **View logs:**
   ```bash
   docker logs -f url-shortener
   ```

### Using Docker Compose

1. **Start services:**
   ```bash
   docker-compose up -d
   ```

2. **View logs:**
   ```bash
   docker-compose logs -f
   ```

3. **Stop services:**
   ```bash
   docker-compose down
   ```

### Docker with Persistent Storage

To persist the SQLite database:

```bash
docker run -d \
  -p 8080:8080 \
  -v $(pwd)/data:/data \
  -e DATABASE_URL=/data/rust_url_shortener.db \
  -e BASE_URL=http://localhost:8080 \
  --name url-shortener \
  rust-url-shortener
```

---

## Cloud Deployment

### AWS (Elastic Beanstalk)

1. **Install EB CLI:**
   ```bash
   pip install awsebcli
   ```

2. **Initialize EB:**
   ```bash
   eb init -p docker rust-url-shortener
   ```

3. **Create environment:**
   ```bash
   eb create production
   ```

4. **Deploy:**
   ```bash
   eb deploy
   ```

### AWS (ECS with Fargate)

1. **Build and push to ECR:**
   ```bash
   aws ecr create-repository --repository-name rust-url-shortener
   docker tag rust-url-shortener:latest <account-id>.dkr.ecr.<region>.amazonaws.com/rust-url-shortener:latest
   docker push <account-id>.dkr.ecr.<region>.amazonaws.com/rust-url-shortener:latest
   ```

2. **Create ECS task definition and service** (use AWS Console or CloudFormation)

### Google Cloud (Cloud Run)

1. **Build and push to GCR:**
   ```bash
   gcloud builds submit --tag gcr.io/<project-id>/rust-url-shortener
   ```

2. **Deploy to Cloud Run:**
   ```bash
   gcloud run deploy rust-url-shortener \
     --image gcr.io/<project-id>/rust-url-shortener \
     --platform managed \
     --region us-central1 \
     --allow-unauthenticated \
     --set-env-vars DATABASE_URL=rust_url_shortener.db,BASE_URL=https://your-service-url.run.app
   ```

### Azure (Container Instances)

1. **Build and push to ACR:**
   ```bash
   az acr create --resource-group myResourceGroup --name myregistry --sku Basic
   az acr build --registry myregistry --image rust-url-shortener:latest .
   ```

2. **Deploy container:**
   ```bash
   az container create \
     --resource-group myResourceGroup \
     --name rust-url-shortener \
     --image myregistry.azurecr.io/rust-url-shortener:latest \
     --dns-name-label rust-url-shortener \
     --ports 8080 \
     --environment-variables DATABASE_URL=rust_url_shortener.db BASE_URL=http://your-domain.com
   ```

### DigitalOcean (App Platform)

1. **Create `app.yaml`:**
   ```yaml
   name: rust-url-shortener
   services:
   - name: web
     dockerfile_path: Dockerfile
     source_dir: /
     github:
       branch: main
       deploy_on_push: true
       repo: your-username/rust-url-shortener
     http_port: 8080
     instance_count: 1
     instance_size_slug: basic-xxs
     env_vars:
     - key: DATABASE_URL
       value: rust_url_shortener.db
     - key: BASE_URL
       value: ${APP_URL}
   ```

2. **Deploy via DigitalOcean console or CLI**

### Heroku

1. **Create `Procfile`:**
   ```
   web: ./target/release/rust-url-shortener
   ```

2. **Deploy:**
   ```bash
   heroku create rust-url-shortener
   heroku buildpacks:set emk/rust
   git push heroku main
   ```

---

## Production Considerations

### Environment Variables

Set these environment variables for production:

```bash
DATABASE_URL=<production-database-url>
BASE_URL=https://yourdomain.com
RUST_LOG=info
HOST=0.0.0.0
PORT=8080
```

### Database Migration to PostgreSQL

For production, consider migrating to PostgreSQL:

1. **Update `Cargo.toml`:**
   ```toml
   diesel = { version = "2.0.4", features = ["postgres", "r2d2", "chrono"] }
   ```

2. **Update environment:**
   ```bash
   DATABASE_URL=postgres://user:password@localhost/rust_url_shortener
   ```

3. **Run migrations:**
   ```bash
   diesel migration run
   ```

### HTTPS/TLS

**Option 1: Reverse Proxy (Recommended)**

Use Nginx or Caddy as a reverse proxy:

```nginx
server {
    listen 80;
    server_name yourdomain.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

**Option 2: Cloud Provider SSL**

Most cloud providers offer built-in SSL/TLS termination.

### Monitoring and Logging

1. **Application Logs:**
   - Use `RUST_LOG=info` for production
   - Consider structured logging with `tracing`

2. **Metrics:**
   - Add Prometheus metrics endpoint
   - Monitor request rate, error rate, latency

3. **Health Checks:**
   - Implement `/health` endpoint
   - Database connectivity check
   - Disk space check

### Performance Tuning

1. **Database Connection Pool:**
   ```bash
   MAX_POOL_SIZE=20
   MIN_POOL_SIZE=5
   ```

2. **Actix Workers:**
   - Set to CPU core count
   - Configure in main.rs

3. **Caching:**
   - Add Redis for frequently accessed URLs
   - Implement TTL for cache entries

### Security Checklist

- [ ] HTTPS enabled
- [ ] Rate limiting implemented
- [ ] Input validation on all endpoints
- [ ] Database backups configured
- [ ] Environment variables secured
- [ ] CORS properly configured
- [ ] Security headers added
- [ ] Regular dependency updates
- [ ] Monitoring and alerting setup

### Backup Strategy

1. **SQLite Backups:**
   ```bash
   # Cron job for daily backups
   0 2 * * * sqlite3 /path/to/database.db ".backup '/path/to/backup/db-$(date +\%Y\%m\%d).db'"
   ```

2. **PostgreSQL Backups:**
   ```bash
   # Automated pg_dump
   0 2 * * * pg_dump rust_url_shortener > /backups/db-$(date +\%Y\%m\%d).sql
   ```

### Scaling Strategies

1. **Horizontal Scaling:**
   - Deploy multiple instances
   - Use load balancer (AWS ALB, Nginx, HAProxy)
   - Session affinity not required (stateless)

2. **Database Scaling:**
   - Read replicas for read-heavy workloads
   - Connection pooling optimization
   - Consider sharding for very large datasets

3. **Caching Layer:**
   - Redis for hot URLs
   - CDN for static content
   - HTTP caching headers

### Cost Optimization

1. **Compute:**
   - Right-size instances
   - Use autoscaling
   - Consider spot/preemptible instances

2. **Database:**
   - Regular cleanup of expired URLs
   - Optimize indexes
   - Archive old data

3. **Networking:**
   - CDN for geographic distribution
   - Compression enabled
   - Optimize payload sizes

---

## Troubleshooting

### Common Issues

1. **Port already in use:**
   ```bash
   # Change PORT in .env or kill existing process
   lsof -ti:8080 | xargs kill
   ```

2. **Database locked:**
   - SQLite doesn't handle high concurrency well
   - Consider PostgreSQL for production

3. **Memory issues:**
   - Increase container memory limits
   - Optimize database queries
   - Check for memory leaks

### Health Check Endpoint

Consider adding a health check endpoint:

```rust
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}
```

---

## Support

For issues and questions:
- GitHub Issues: [repository-url]/issues
- Documentation: [repository-url]/docs
