# Multithreaded-Web-Server
This project implements a multithreaded HTTP web server in Rust with a custom thread pool. It handles multiple client connections concurrently by reusing a fixed number of worker threads, improving performance compared to spawning a new thread for every request.

---

## ✅ Features

- Handles HTTP requests on `127.0.0.1:7878`
- Routes:
  - `/` → Returns `hello.html`
  - `/sleep` → Simulates slow response (5s delay)
  - Any other path → Returns `404.html`
- **Custom Thread Pool** for concurrent request handling
- **Graceful Shutdown** after serving 2 connections
- Logs worker activity for clarity

---

## 🛠 How It Works

- **TCP Listener** accepts incoming connections on `127.0.0.1:7878`
- Each connection is submitted as a **job** to the **thread pool**
- Workers pick up jobs from the channel and execute them
- After processing **2 connections**, the server shuts down because take(2) is Initalized for Optimization.
- Thread pool ensures a **graceful exit** by joining all worker threads

---

## ▶ Example Output
---

## 🛠 How It Works

- **TCP Listener** accepts incoming connections on `127.0.0.1:7878`
- Each connection is submitted as a **job** to the **thread pool**
- Workers pick up jobs from the channel and execute them
- After processing **2 connections**, the server shuts down
- Thread pool ensures a **graceful exit** by joining all worker threads

---

## ▶ Example Output
---

## 🛠 How It Works

- **TCP Listener** accepts incoming connections on `127.0.0.1:7878`
- Each connection is submitted as a **job** to the **thread pool**
- Workers pick up jobs from the channel and execute them
- After processing **2 connections**, the server shuts down
- Thread pool ensures a **graceful exit** by joining all worker threads

---
## Project Structure 

MULTITHREADED_WEB_SERVER_IN_RUST/
│
├── src/                     # Source code directory
│   ├── lib.rs              # Library file (ThreadPool implementation)
│   └── main.rs             # Entry point for the web server
│
├── .gitignore              # Git ignore file
├── 404.html                # HTML page for 404 NOT FOUND
├── hello.html              # HTML page for successful response
├── Cargo.lock              # Cargo lock file (auto-generated)
├── Cargo.toml              # Cargo configuration file
└── README.md               # Project documentation

---

##  Example Output
<img width="720" height="396" alt="Screenshot 2025-08-02 at 11 52 48 PM" src="https://github.com/user-attachments/assets/8556c1d4-6388-47f9-b755-70d9ede3078f" />


---

## How to Run

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/rust-multithreaded-webserver.git
```
### 2. Direct to the folder
```bash
cd rust-multithreaded-webserver
```
### 3. Build & Run
```bash
cargo run
```

---

## Test in Browser
- http://127.0.0.1:7878/ → Loads hello.html
- http://127.0.0.1:7878/sleep → Waits 5 seconds, then loads hello.html
- Any other path → 404.html

---




