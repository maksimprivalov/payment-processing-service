# Mini Payment Processing System
##### Microservice-based application using Rust.

## 1. Problem Description
Modern financial systems must process monetary transactions in a **reliable**, **secure**, and **consistent manner**. Even in simplified environments, payment systems must ensure **correctness of balances**, **prevent duplicate execution of requests**, and **provide traceability of all operations**.

The goal of this project is to design and implement a **mini payment processing system** that demonstrates the fundamental principles of *financial transaction processing* using a microservice architecture. The system focuses on **correctness**, **separation of responsibilities**, and basic **fault-prevention mechanisms** rather than **real-world banking protocols**.

## 2. Overview of the Solution
The proposed solution is a *microservice-based backend system* implemented in Rust.
Each microservice has a clearly defined responsibility and its own database, following the principles of *loose coupling and high cohesion*.

The system supports:
- user authentication and authorization,
- management of financial accounts,
- execution of money transfers,
- consistent bookkeeping using double-entry accounting,
- auditing and logging of all relevant operations.

Inter-service communication is implemented via RESTful APIs, while access to protected resources is secured using JWT-based authentication.
## 3. System Architecture
The system consists of the following microservices:
### 3.1. Authentication & User Service

Responsibilities:
- user registration,
- user authentication,
- issuing and validation of JWT tokens,
- basic authorization support

Data model *approximately*:
- User (id, email, password_hash, status)

Interactions:
- Frontend communicates with this service for login and registration.
- Other services validate incoming requests using JWT tokens issued by this service.

### 3.2. Account Service
Responsibilities:
- management of user financial accounts,
- storing and updating account balances,
- validating whether sufficient funds are available for transactions.

Data model *approximately*:
- Account (account_id, user_id, balance, currency)

Interactions:
- *Payment Service* queries *Account Service* **to verify account balance**.
- *Ledger Service* references account identifiers during bookkeeping operations.

### 3.3. Payment Service
**Responsibilities**:
- processing money transfer requests between accounts,
- validation of business rules (e.g., prevention of negative balances),
- coordination of transaction execution across services,
- idempotent request handling to prevent duplicate transaction execution.

**Data model** *approximately*:
- Payment (payment_id, from_account, to_account, amount, status)

**Interactions**:
- communicates with *Account Service* to validate balances,
- invokes *Ledger Service* to record financial transactions,
- reports transaction outcomes to the *Audit Service*.

### 3.4. Ledger Service (Double-Entry Accounting)
**Responsibilities**:
- maintaining a consistent financial ledger,
- implementing double-entry accounting principles,
- ensuring that each transaction is recorded as a debit and a credit entry.

**Data model** *approximately*:
- LedgerEntry (entry_id, account_id, debit, credit, timestamp)

Each completed payment generates **two ledger entries**:
- a debit entry for the sender's account,
- a credit entry for the receiver's account.

### 3.5. Audit / Transaction Log Service
**Responsibilities**:
- logging all transaction attempts and system events,
- storing historical data for traceability and debugging,
- enabling post-analysis of system behavior.

**Data model** *approximately*:
- AuditEvent (event_id, service_name, action, status, timestamp)

## 4. Communication and Data Management

- Services communicate exclusively via REST APIs.
- JWT tokens are used to authorize requests between services.
- Each microservice owns its **separate database** and does not directly access data from other services.
- Data consistency is ensured through controlled service interactions rather than shared databases.
## 5. Technologies Used

- Rust (primary programming language)
- Web framework: Axum (or Actix-web)
- Database: PostgreSQL (one per service)
- Authentication: JWT
- Containerization: Docker & Docker Compose
- Data serialization: JSON

## 7. Project Limitations and Conclusion

This project is *intended for educational purposes only* and does not implement:
- real-world banking or payment protocols,
- regulatory compliance standards (e.g., PCI-DSS),
- real-time settlement systems.

The focus is on demonstrating architectural and implementation principles rather than production-level financial systems.

---
Overall the implemented system provides a clear and structured example of a simplified payment processing platform. Through the use of microservices, strict responsibility separation, and fundamental fintech concepts, the project demonstrates practical application of backend development and distributed system principles using Rust.

Technologies may *a bit* change while implementing.