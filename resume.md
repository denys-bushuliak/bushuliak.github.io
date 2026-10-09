<style>
    #resume-link{
        color: var(--ink) !important;
        border-bottom: 1px solid var(--ink) !important;
    }
</style>

# About Me

<div class="stripe">
<div class="wrap">

<p class="statement">A software engineer and solution architect with 20+ years turning business requirements into <strong>robust, scalable systems</strong> — ERP, CRM, payments, and service delivery platforms.</p>

I build distributed systems and modern web applications, with a strong background in backend development using Rust, PHP, Java, and Node.js, and frontend work in React and Angular. I prioritize clean, testable code, and implement Hexagonal Architecture, CQRS, microservices, and Domain-Driven Design.

I am an AI enthusiast and developer of agentic systems, building AI-powered products and autonomous agent workflows.

</div>
</div>

<div class="stripe stripe--alt">
<div class="wrap">

<div class="group">

### Experience

<p class="current-role"><a href="https://netconomy.net">Netconomy</a> — <strong>Senior Software Engineer (L3)</strong> · 2026 — present</p>

NETCONOMY is a European digital agency delivering digital strategy, cloud-native application development, commerce platforms, and AI/ML solutions on SAP CX, Google Cloud, and Microsoft Azure.

Previously — Principal Software Engineer and solution architect across ERP, travel, mobility, and edtech product teams.

</div>

<div class="group">

### Education

- **Degree** Master of Science in Mechanical Engineering
- **Specialization** Automation of Manufacturing Processes using Information and Computer Technologies
- **Institution** Vinnytsia National Technical University
- **Year** 2009

</div>

<div class="group">

### Known Languages

- **English** reading C2, listening and speaking B2 ([EF SET](https://cert.efset.org/WSdoAM))
- **Ukrainian** Native
- **Russian** Native
- **Kurmanji** B1
- **Turkish** A1

</div>

</div>
</div>

<div class="stripe">
<div class="wrap">

<div class="group">

### Interests

Committed to continual learning, I regularly read white papers from ACM, IEEE, VLDB, and beyond. I mentor newcomers on their journey as developers and volunteer to teach aspiring developers.

- Database internal architecture, data storing formats, InMemory DBMS
- OS development and hypervisors
- WebAssembly
- Functional Programming

</div>

<div class="group">

### Volunteering

- I teach programming and server application architecture for free.
- I created and maintain a website for learning the Kurdish language (Kurmanji dialect) at [https://denys-bushuliak.github.io/kurmanci](https://denys-bushuliak.github.io/kurmanci).

</div>

</div>
</div>


# Projects

<div class="stripe">
<div class="wrap">

<div class="project" id="now-you-know">

## Now You Know

A scalable online education platform offering video courses across a wide range of subjects, with detailed analytics for user engagement, personalized content delivery, and training status tracking across social media. Designed with a microservices architecture using Rust and Python, the platform ensures high performance and reliability through asynchronous processing, data pipelines, and advanced analytics tooling. Cloud infrastructure deployed and monitored using AWS services and Kubernetes.

**Responsibilities:** Designing the internal system architecture using Hexagonal and Service-Oriented approaches; Developing the product management module for course creation, updates, and tracking; Implementing analytical data processing to monitor user training status across social media platforms; Designing and managing data storage systems for structured and semi-structured data; Deploying and maintaining cloud infrastructure with Kubernetes and AWS; Configuring business analytics tools and ensuring continuous monitoring and observability; Managing CI/CD processes and supporting infrastructure scalability and reliability.

- Rust (Tokio, Cargo Lambda, Workspace, concurrency and multi-threading)
- Python
- Node.js
- TypeScript
- HTML5/CSS
- Custom SQL Dialect
- PostgreSQL
- MongoDB
- React.js
- Apache Airflow
- DataFusion
- Arrow
- Parquet
- AWS (EKS, S3, SNS, SQS, EFS, ElastiCache, CloudFront, EventBridge, CloudWatch, EC2, Lambda)
- Scaleway Cloud
- Kubernetes
- CI/CD
- Pest.rs

</div>

<div class="project" id="erp-footwear">

## ERP System for Footwear Manufacturing

An enterprise resource planning (ERP) system for a shoe manufacturing company, automating operations across the full production and delivery lifecycle: real-time tracking of materials, pricing automation, supplier settlements, product logistics, and reseller billing. Built with event-driven architecture and CQRS for reliability, traceability, and modular scaling.

**Responsibilities:** Designing system architecture using Hexagonal Architecture, Event Sourcing, and CQRS principles; Implementing automated tracking of production resources from material intake to delivery; Developing pricing automation logic and financial workflows for supplier and reseller management; Leading and mentoring the development team; Configuring DevOps processes using Docker Compose and GitHub Actions; Managing cloud storage and database infrastructure.

- Node.js
- TypeScript
- JavaScript
- GraphQL
- ReactJS
- Hexagonal Architecture
- Event Sourcing
- CQRS
- ArangoDB
- MongoDB
- AWS S3
- GitHub Actions
- Docker Compose

</div>

</div>
</div>

<div class="stripe stripe--alt">
<div class="wrap">

<div class="project" id="erp-travel">

## ERP System for Travel Operator

A modern ERP system for a travel company managing tours, bookings, pricing, and operational workflows. The project included full migration from a legacy system, with a focus on scalability, modularity, and performance. Built using microservices and Domain-Driven Design (DDD) principles, with secure APIs and flexible access control via a custom ABAC service.

**Responsibilities:** Designing a new ERP architecture with DDD and microservices principles; Planning and executing seamless migration from the legacy system; Selecting the technology stack and defining core service contracts (GraphQL/REST APIs); Leading the development team and ensuring code quality; Implementing key modules and coordinating integration between services; Managing CI/CD pipelines and deploying the system using OpenShift.

- Node.js
- PHP
- TypeScript
- JavaScript
- HTML/CSS
- ReactPHP
- RoadRunner
- GraphQL
- REST
- JWT
- Custom ABAC Service
- Redis
- MongoDB
- GitLab CI/CD
- Jenkins
- OpenShift

</div>

<div class="project" id="taxi-platform">

## Uber-like Taxi Service Platform

A full-featured taxi service platform supporting ride booking, real-time communication, driver-client matching, and billing operations. The system included mobile apps for drivers and clients, a web-based back-office, and a microservices backend. Real-time functionality was powered by socket-based communication, with scalable deployments via Docker Swarm.

**Responsibilities:** Designing the full system architecture — backend, back-office frontend, and mobile applications; Leading the development team and overseeing feature planning, implementation, and integration; Developing real-time communication features using WebSockets (Socket.io); Implementing secure authentication and authorization flows using JWT; Building and deploying microservices for core business logic and billing; Managing mobile development using React Native and Expo; Setting up infrastructure using Docker Swarm.

- Node.js
- PHP
- TypeScript
- JavaScript
- Laravel
- Socket.io
- JWT
- ReactJS
- React Native
- Expo Framework
- MongoDB
- Redis
- Pub/Sub
- Docker Swarm

</div>

<div class="project" id="safefrontier">

## SafeFrontier

A cloud-based service delivery platform supporting business operations across messaging, billing, information planning, and customer support. The platform featured a custom-built email service, contextual help modules, and push notification integration for Apple, Android, and Blackberry. Built with extensibility in mind — SDK toolkits and developer documentation included.

**Responsibilities:** Gathering business requirements and converting them into technical specifications for multiple platform modules; Designing and developing a custom email service with an SDK and supporting documentation; Implementing modules for client and sub-client billing, knowledge base, contextual help, and information planning; Developing and integrating push notification services; Implementing real-time communication using WebSockets and TCP/IP sockets; Managing data storage and operations via MySQL.

- PHP
- JavaScript
- HTML/CSS
- Apple Push Notification Service
- Android Push Service
- Blackberry
- WebSockets
- TCP/IP Sockets
- MySQL

</div>

</div>
</div>


# Skills

<div class="stripe">
<div class="wrap">

<div class="group">

### AI and Agent Systems

- LLMs (OpenAI, Anthropic)
- Agentic Systems
- RAG
- MCP
- Vector DBs

</div>

<div class="group">

### Programming Languages and DSLs

- Rust
- Go
- JavaScript/TypeScript
- F#
- Java
- C#
- PHP
- SQL
- Cypher QL
- ArangoDB QL
- Delphi
- Custom DSLs
- ANTLR
- PEG Grammar
- BNF/BNFC

</div>

<div class="group">

### Architectural Patterns and Concepts

- Category Theory
- Concurrency
- CQRS
- Developer Experience (DX)
- Domain-Driven Design (DDD)
- Event Sourcing
- Finite State Machines
- Functional Programming
- Flux Architecture Pattern
- Hexagonal Architecture (Port and Adapter)
- Micro Frontend
- Microservices
- Multithreading
- MVC/HMVC
- Onion Architecture
- Object Oriented Programming
- Pub/Sub
- RBAC/ABAC
- Service-Oriented Architecture (SOA)

</div>

<div class="group">

### API Development

- Any Custom protocol
- GraphQL
- REST
- SOAP
- WebSockets
- XML-RPC

</div>

</div>
</div>

<div class="stripe stripe--alt">
<div class="wrap">

<div class="group">

### Frontend Technologies

- AngularJS/Angular
- React Native
- ReactJS
- Redux
- RxJS
- Vue.js
- WebAssembly
- XState

</div>

<div class="group">

### Backend Technologies

- Doctrine
- Express
- GraphQL (Apollo, Yoga, Vanilla)
- JDBC Type 3 Driver Implementation
- Laravel
- Pest.rs
- ReactPHP
- RedBeanPHP
- Socket.io
- Symfony
- Tokio
- WebAssembly

</div>

<div class="group">

### Testing

- Behavior-Driven Development (BDD)
- Cucumber Framework
- Test-Driven Development (TDD)

</div>

<div class="group">

### Tools and IDEs

- Git
- GitHub Actions
- GitLab CI/CD
- JetBrains tools
- Jest
- Playwright
- React Testing Library
- SVN
- VIM
- VSCode

</div>

<div class="group">

### Database Technologies

- Apache Arrow
- Apache DataFusion
- ArangoDB
- Avro
- ClickHouse
- MongoDB
- MySQL
- Neo4J
- Parquet
- PostgreSQL
- Presto
- Trino
- Redis & Redis Streams
- SQLite

</div>

<div class="group">

### Cloud and DevOps

- Apache Airflow
- AWS (EKS, SQS, SNS, Lambda, IAM)
- Docker
- Docker Swarm
- Kubernetes
- MinIO
- Scaleway Cloud
- Tableau Server

</div>

</div>
</div>


# Recommendation Letters

<div class="stripe">
<div class="wrap">

<div class="letter" id="devbrother">

### DevBrother LLC

<p class="date">March 2, 2026</p>

Denys is a responsible and proactive Rust Engineer with strong ownership of his work. He consistently delivers high-quality solutions and contributes valuable ideas to improve architecture, performance, and development processes. He is genuinely invested in both the product and the team's success.

He has solid expertise in distributed and high-load systems and approaches development in a structured, thoughtful way. Denys actively participates in technical discussions, carefully evaluates trade-offs, and collaborates effectively in pair programming to optimize performance.

Beyond his technical skills, Denys is open, positive, and easy to work with. He communicates complex ideas clearly and contributes to a culture of knowledge sharing by leading internal tech talks.

I confidently recommend Denys as a dedicated engineer and strong team player who would bring real value to any company.

<p class="signature">Sincerely,<br>Anna Yevstiefieieva<br>HR Manager, DevBrother</p>

<a class="doc" href="./assets/letters/DB_Recommendation_Letter.pdf">Document</a>

</div>

<div class="letter" id="devbrother">

### InRemote LLC

<p class="date">March 29, 2024</p>

To whom it may concern, I have the honor to recommend Denys Bushuliak in the light of his impressive achievements and capabilities in software development.

Throughout our collaboration at Safe Frontier, I have had the opportunity to witness Denys's professional growth. He has demonstrated remarkable abilities in planning and developing complex full stack applications. Through his work and adherence to best practices such as Test-Driven Development and system decomposition, the company has achieved significant improvement in software development life cycle and product quality.

Particularly noteworthy is his ability to independently solve problems in various and new domains. He has shown responsibility in developing complex systems such as CMS, knowledge base system, mail server, client SDK, backend mobile device management system, invoicing and billing system, contextual help system, and managed task scheduling system.

It is important to highlight that this entire set of systems was integrated into the company's product line, which emphasizes the responsibility for the quality of implementation and performance. Additionally, Denys implemented significant improvements of the security policy execution engine, enabling the company to achieve even higher levels of productivity and efficiency.

I can confidently say that Denys is a valuable asset to any project or company. His talent, professionalism, and ability to deliver results make him a reliable colleague.

I recommend Denys Bushuliak without any reservations and with great enthusiasm, and I am confident that he will be an additional asset to your team.

<p class="signature">Sincerely,<br>Andrii Bardachenko<br>CTO, InRemote LLC</p>

<a class="doc" href="./assets/letters/InRemote_Recommendation_letter.pdf">Document</a>

</div>

</div>
</div>

<div class="stripe stripe--alt">
<div class="wrap">

<div class="letter" id="inremote">

### Now You Know

<p class="date">July 3, 2024</p>

To whom it may concern, I am pleased to write a letter of recommendation for Denys Bushulyak. As the Co-Founder & CTO at Now You Know, I have had the pleasure of working closely with Denys from July 2021, and I can confidently attest to his exceptional abilities, drive for excellence, and an impressive passion for innovation and technology.

Since joining our team, Denys has been an exceptional asset, significantly enhancing our operations with a blend of technical brilliance and innovative thinking. As a software engineer, Denys excels in creating solutions that not only address our immediate challenges but also streamline our processes for better efficiency. This passion for technology and its application to improve our workflows has been both inspirational and transformative, making Denys a standout contributor to our software engineering initiatives.

Beyond his technical skills, Denys stands out for his strategic vision and his capacity to identify and capitalize on opportunities for improvement. His proactive approach to adopting and implementing new technologies has significantly advanced our company's objectives, setting new standards for excellence within our team.

Denys's continuous pursuit of professional growth, combined with his commitment to keeping abreast of the latest industry trends, demonstrates his dedication to excellence in the field. His forward-thinking nature and ability to adapt to and lead through technological advancements make him a valuable asset to any team or project.

In conclusion, Denys Bushulyak stands out as a remarkable software engineer, whose contributions have consistently propelled innovation and set new standards of excellence within our team. His ability to transform challenges into opportunities for advancement is sure to bring invaluable insights and results wherever his career path leads.

<p class="signature">Sincerely,<br>Ofer Feder<br>Co-Founder & CTO, Now You Know</p>

<a class="doc" href="./assets/letters/Now_You_Know_Recommendation_Letter.pdf">Document</a>

</div>

</div>
</div>


