# Rust-based Ultimate Support Hub (rush)

A simple Q&A server implemented in Rust with [warp](https://github.com/seanmonstar/warp) framework.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Features


## Installation

Ensure you have Rust and Cargo installed on your system. You can install them by following the instructions on [rust-lang.org](https://www.rust-lang.org/learn/get-started).

Clone this repository to your local machine:

```sh
git clone https://github.com/haipro287/rush.git
cd rush
```

## Usage

### Running the Project

Follow these steps to run the project:

1. Navigate to the project directory in your terminal.

2. Create `.env` from `.env.example` and fill out all the values

```sh
cp .env.example .env
```

3. Build the project using Cargo:

```sh
cargo build
```

4. Run the project:

```sh
cargo run
```

### Running with Docker

1. Navigate to the project directory in your terminal.

2. Create `.env.docker` from `.env.example` and fill out all the values

```sh
cp .env.example .env.docker
```

3. Run `docker-compose.yml` to start Postgres database and our server

```sh
docker compose up -d --build
```

### Running Tests

To run tests, use Cargo's built-in testing feature:

```sh
cargo test
```

## Contributing

Contributions are welcome! If you'd like to contribute to this project, feel free to submit a pull request. Please follow the [Contributing Guidelines](CONTRIBUTING.md).

## License

This project is licensed under the [MIT License](LICENSE).