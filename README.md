# Todo.rs

> A simple task management backend with authorization and email verification, powered by Axum and SQLx.

## Features

### Task Management:

- Create, read, update, and delete tasks.
- Mark tasks as completed.

### User Authentication:

- User registration and login with JWT-based authentication.
- Password hashing for secure storage.

### Email Verification:

- Send verification emails to users upon registration.
- Verify email addresses using a unique token.

### Database Integration:

- Built with SQLx for type-safe SQL queries.
- Supports PostgreSQL as the primary database.

### API Framework:

- Powered by Axum, a fast and ergonomic web framework for Rust.

## Technologies Used

- [Axum](https://github.com/tokio-rs/axum): A web framework for Rust
- [SQLx](https://github.com/launchbadge/sqlx): Async SQL toolkit for Rust
- [Tokio](https://github.com/tokio-rs/tokio): Asynchronous runtime for Rust
- [JSON Web Tokens (JWT)](https://github.com/Keats/jsonwebtoken): For secure user authentication
- [argon2](https://github.com/RustCrypto/password-hashes): For password hashing
- [Lettre](https://github.com/lettre/lettre): For sending emails
- [PostgreSQL](https://www.postgresql.org): Relational database for storing tasks and user data

## License

This project is licensed under the MIT License.
