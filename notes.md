# Backend for Usher

## TODO List
- <b>Rust</b>
- <s>ORM or not? Query constructor? SQLX?</s><b>sqlx</b>
- <s>Webserver, actix/warp/rocket? (actix is preferred)</s> <b>Axum</b>
- Login phase, email/phone number/google/apple/facebook?
- How to handle ride creation process? TODO Sequence diagrams
- Payments? stripe/paypal/other?

## API Design
Probably I am more comfortable in designing API based on JSON instead of GET/POST params

### User handling
- Sign up
    - Customer
    - Driver
- Login
- Sign out

### Actions
- Create a ride (Customer)
- Accept a ride (Driver)
- End a ride (Customer, Driver, both?)
- Add review to a ride

### Docs
- Comments on code
- Flow digrams
- Documentation in Markdown  (while writing code)
- <b>-> Test <-</b>
