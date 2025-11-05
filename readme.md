# Fullstack Review Application

This project is a fullstack application for managing and searching product reviews. It consists of a Rust-based backend (using Axum and SPFRESH for search) and a Leptos-based frontend. The backend handles review storage and search, while the frontend provides a user interface.
#demo

https://github.com/user-attachments/assets/7253b84a-c91d-4189-9b24-c72b6cdd7c83


## Prerequisites

- Docker and Docker Compose
- Git
- Python 3 (for seeding data)
- Rust (if building locally without Docker)

## Project Structure

- [`backend`](backend ): Rust backend with SPFRESH integration for search functionality.
- [`frontend`](frontend ): Leptos frontend.
- [`seed.py`](seed.py ): Script to seed review data from CSV.
- [`TestReviews.csv`](TestReviews.csv ): Sample review data.
- [`docker-compose.yml`](docker-compose.yml ): Orchestrates backend and frontend services.

## Setup Instructions

### 1. Clone the Repository and Initialize Submodules

Clone the repository and initialize the backend submodule:

```sh
git clone <repository-url>
cd fullstack
git submodule update --init --recursive
```

This pulls the backend code from the submodule at `https://github.com/inwchamp1337/backend-spfresh`.

### 2. Build SPFRESH in Backend

SPFRESH is a custom search engine used by the backend. It must be built before building the backend application. The build process is handled via the `spfresh-docker` directory in the backend.

Navigate to the backend directory:

```sh
cd backend
```

#### Prerequisites for SPFRESH Build

- Ensure you have Docker installed, as the build uses Docker containers.
- The build scripts assume a Linux environment (or WSL on Windows).

#### Detailed SPFRESH Build Steps

1. **Navigate to spfresh-docker**:

   ```sh
   cd spfresh-docker
   ```

2. **Install Dependencies**:

   Run the script to install required dependencies:

   ```sh
   ./scripts/install-deps.sh
   ```

   This script installs necessary packages like GCC, CMake, etc., inside a Docker container.

3. **Clone and Initialize SPFRESH Source**:

   ```sh
   ./scripts/clone-and-init.sh
   ```

   This clones the SPFRESH repository and initializes submodules if needed.

4. **Build RocksDB**:

   SPFRESH depends on RocksDB. Build it first:

   ```sh
   ./scripts/build-rocksdb.sh
   ```

   This compiles RocksDB from source.

5. **Build ISAL (Intel Storage Acceleration Library)**:

   ```sh
   ./scripts/build-isal.sh
   ```

   This builds the ISAL library for optimized compression.

6. **Build SPDK (Storage Performance Development Kit)**:

   ```sh
   ./scripts/build-spdk.sh
   ```

   This compiles SPDK for high-performance storage.

7. **Build SPFRESH**:

   Finally, build SPFRESH itself:

   ```sh
   ./scripts/build-spfresh.sh
   ```

   This script compiles SPFRESH with all dependencies. It may take some time.

8. **Verify Build**:

   Check if the build artifacts are created in the expected directories (e.g., `build/` or `install/` depending on the scripts).

If any step fails, ensure Docker is running and you have sufficient permissions. You may need to run as root or adjust Docker settings.

Once SPFRESH is built, return to the backend root:

```sh
cd ..
```

### 3. Build the Backend

With SPFRESH built, build the Rust backend:

```sh
cargo build --release
```

This compiles the backend application, linking against the built SPFRESH library.

If building with Docker (recommended for consistency):

```sh
docker build -f Dockerfile.api -t rust-backend .
```

### 4. Build the Frontend

Navigate to the frontend directory:

```sh
cd ../frontend
```

Build the Leptos frontend:

```sh
trunk build --release
```

Or use Docker:

```sh
docker build -t leptos-frontend .
```

### 5. Run the Application

Return to the root directory and use Docker Compose to run both services:

```sh
docker-compose up --build
```

- Backend will be available at `http://localhost:8000`.
- Frontend will be available at `http://localhost:3000`.

The frontend is configured to connect to the backend via `BACKEND_URL=http://backend` and `BACKEND_PORT=8000` in the Docker Compose environment.

### 6. Seed Data

To populate the backend with sample reviews, use the provided seed script:

```sh
python seed.py --file TestReviews.csv --url http://localhost:8000/reviews
```

- This reads [`TestReviews.csv`](TestReviews.csv ), maps `class` to ratings (0 -> 1-2, else -> 3-5), and posts reviews to the backend.
- Use `--dry-run` to preview payloads without sending.
- Adjust `--limit` and `--delay` as needed.

## API Endpoints

- `POST /reviews`: Add a new review (expects JSON with `review_title`, `review_body`, [`product_id`](../../../../d:/YEARRIGHT/New folder/fullstack/seed.py ), `review_rating`).
- Search endpoints (via SPFRESH integration) are handled in the search module.

## Configuration

- Backend config is in [`backend/config.json`](backend/config.json ).
- Environment variables for frontend are in [`frontend/.env`](frontend/.env ).

## Troubleshooting

- If SPFRESH build fails, ensure all dependencies are installed and Docker has access to the host filesystem.
- For Rust compilation issues, check that SPFRESH libraries are correctly linked (may require adjusting `build.rs`).
- If services don't start, check Docker logs: `docker-compose logs`.

## Contributing

1. Follow the coding instructions in the attached instruction files for modifying code.
2. Ensure changes align with the patterns in `*-post-r-pm-*`, `*-pre-r-pm-*`, etc., as per the instructions.

For more details, refer to [`backend/README.md`](backend/README.md ) and `frontend/README.md`.
