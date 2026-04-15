rag-infra-system/
│
├── services/                         # 🚀 All deployable microservices
│   ├── api-gateway/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── transport/
│   │   │   ├── middleware/          # rate limit, auth
│   │   │   └── config/
│   │   ├── Cargo.toml
│   │   └── Dockerfile
│   │
│   ├── rag-service/                 # 🧠 CORE SERVICE
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── application/         # pipeline
│   │   │   ├── services/            # cache, coalescing
│   │   │   ├── clients/             # embedding, llm, vector db
│   │   │   ├── config/
│   │   │   └── wiring/              # dependency injection
│   │   ├── Cargo.toml
│   │   └── Dockerfile
│   │
│   ├── embedding-service/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── transport/
│   │   │   ├── service/
|   |   |   ├── wiring/              # dependency injection
│   │   │   └── config/
│   │   ├── Cargo.toml
│   │   └── Dockerfile
│   │
│   ├── llm-service/
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── transport/             # HTTP handlers
│   │   │   ├── providers/           # OpenAI, local, etc.
|   |   |   ├── wiring/              # dependency injection
│   │   │   └── service/             # LLM-specific logic
│   │   ├── Cargo.toml
│   │   └── Dockerfile
│
│
├── shared/                          # 📦 Shared libraries (NO NETWORK)
│   ├── domain/
│   │   ├── models.rs
│   │   ├── errors.rs
│   │   └── context.rs
│   │
│   ├── resilience/
│   │   ├── retry.rs
│   │   ├── circuit_breaker.rs
│   │   ├── timeout.rs
│   │   └── mod.rs
│   │
│   ├── observability/
│   │   ├── metrics.rs
│   │   ├── tracing.rs
│   │   └── logger.rs
│   │
│   └── utils/
│       └── hashing.rs
│
│
├── infrastructure/                  # 🔌 External system adapters/config
│   ├── kafka/
│   │   ├── producer.rs
│   │   ├── consumer.rs
│   │   └── topics.rs
│   │
│   ├── cache/
│   │   ├── redis.rs
│   │   └── config.rs
│   │
│   ├── vector-db/
│   │   └── client.rs
│
│
├── deployments/                     # 🚀 Deployment configs
│   ├── docker-compose.yml
│   │
│   ├── kubernetes/
│   │   ├── api-gateway.yaml
│   │   ├── rag-service.yaml
│   │   ├── embedding.yaml
│   │   ├── llm.yaml
│   │   ├── redis.yaml
│   │   ├── kafka.yaml
│   │   └── vector-db.yaml
│   │
│   └── env/
│       ├── dev.env
│       └── prod.env
│
│
├── scripts/                         # 🛠️ Dev utilities
│   ├── build.sh
│   ├── run_local.sh
│   └── seed_data.sh
│
│
├── Cargo.toml                       # 🧠 Workspace root
└── README.md

🧠 Inside rag-service (VERY IMPORTANT)
rag-service/src/
│
├── application/
│   └── rag_pipeline.rs
│
├── services/
│   ├── cache_service.rs
│   ├── coalescing.rs
│
├── clients/
│   ├── embedding_client.rs
│   ├── llm_client.rs
│   ├── vector_client.rs
│   ├── cache_client.rs
│   └── kafka_client.rs
│
├── wiring/
│   └── container.rs