# 100xbackend-assignment-express-rust

This is a project to learn writing HTTP server in rust using actix-web framework and benchmark it's performance with express 

There is a single integration test written for testing the backend 

Change the directory to any sub folder to test the backend  

## Quick Start

1. Install dependencies  
  ```bash
  cd http-express && pnpm install 
  ````
  ```bash
  cd http-rust && cargo build 
   ```

2. Start backend 
```bash
pnpm dev 
   ```
```bash
cargo run 
  ```

3. Testing the backend 

start either of the backend and run the test script 
```bash
cd integration-test && pnpm test 
```
