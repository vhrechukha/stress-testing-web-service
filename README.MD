# Stress Testing Web Service

This project is a Rust-based web service using PostgreSQL. The service provides endpoints to manage employees. We use Siege for stress testing to measure the performance of the web service under different load conditions.

## Project Setup

### Prerequisites

- Rust
- Docker and Docker Compose
- Siege

### Getting Started

1. **Set up environment variables**:
    Create a `.env` file in the root directory and configure variables:
    ```dotenv
    cp .env.example .env
    ```

2. **Run the migration**:
    ```bash
    diesel setup
    DATABASE_URL=<DATABASE_URL> diesel migration run
    ```

3. **Build and run Docker containers**:
    ```bash
    docker-compose build --no-cache
    docker-compose up -d
    ```

### Endpoints

- **GET /employees**: Retrieve a list of employees.
- **POST /employees**: Add a new employee.

### Stress Testing with Siege

1. **Already created `testing/urls.txt`. If more test cases are needed, add them here**:
    ```txt
    http://localhost:8080/employees POST application/json {"name": "John Doe", "age": 30, "position": "Developer"}
    http://localhost:8080/employees
    ```

2. **Run Siege with different concurrency levels**:
    ```bash
    ./testing/run_siege.sh
    ```

### Siege Test Results
File will be saved under folder `testing/results` with corresponding timestamp.


Results from file `siege_results_2024-08-05_08-30-57.csv`.

| Date                | Concurrency | Resource Availability | Avg Response Time | Throughput |
|---------------------|-------------|-----------------------|-------------------|------------|
| 2024-08-05_08-30-57 | 10          | 99.99                 | 0.01              | 537.77     |
| 2024-08-05_08-30-57 | 25          | 99.98                 | 0.04              | 538.76     |
| 2024-08-05_08-30-57 | 50          | 100.00                | 0.09              | 539.06     |
| 2024-08-05_08-30-57 | 100         | 99.85                 | 0.14              | 533.23     |

- **Date**: The timestamp when the test was run
- **Concurrency**: The number of simultaneous users simulated during the test
- **Resource Availability**: The percentage of requests that were successfully handled
- **Avg Response Time**: The average time taken by the server to respond to a request
- **Throughput**: The number of requests processed per second

## License

This project is licensed under the MIT License.
