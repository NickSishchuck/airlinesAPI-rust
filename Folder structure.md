VIEW IN SOURCE MODE!!!

airline_api/
├── .env                                  # Environment variables
├── .gitignore                           # Git ignore file
├── Cargo.toml                           # Project dependencies
├── Dockerfile                           # Docker configuration
├── docker-compose.yml                   # Docker compose configuration
├── migrations/                          # SQLx migrations
│   └── 20240418000001_initial_schema.sql # Initial database schema
├── src/                                 # Source code directory
│   ├── main.rs                          # Application entry point
│   ├── error.rs                         # Central error handling
│   ├── auth/                            # Authentication module
│   │   └── mod.rs                       # JWT implementation
│   ├── db/                              # Database connection
│   │   └── mod.rs                       # Database setup
│   ├── handlers/                        # Request handlers
│   │   ├── mod.rs                       # Handlers module exports
│   │   ├── aircraft.rs                  # Aircraft handlers
│   │   ├── auth.rs                      # Auth handlers
│   │   ├── crew.rs                      # Crew handlers
│   │   ├── crew_member.rs               # Crew member handlers
│   │   ├── flight.rs                    # Flight handlers
│   │   ├── flight_seat.rs               # Flight seat handlers
│   │   ├── route.rs                     # Route handlers
│   │   ├── ticket.rs                    # Ticket handlers
│   │   └── user.rs                      # User handlers
│   ├── middleware/                      # Custom middleware
│   │   ├── mod.rs                       # Middleware module exports
│   │   └── validator.rs                 # Request validation middleware
│   ├── models/                          # Data models
│   │   ├── mod.rs                       # Models module exports
│   │   ├── aircraft.rs                  # Aircraft model
│   │   ├── crew.rs                      # Crew model
│   │   ├── crew_member.rs               # Crew member model
│   │   ├── flight.rs                    # Flight model
│   │   ├── flight_seat.rs               # Flight seat model
│   │   ├── route.rs                     # Route model
│   │   ├── ticket.rs                    # Ticket model
│   │   └── user.rs                      # User model
│   ├── routes/                          # Route definitions
│   │   ├── mod.rs                       # Main router setup
│   │   ├── aircraft.rs                  # Aircraft routes
│   │   ├── auth.rs                      # Auth routes
│   │   ├── crews.rs                     # Crew routes
│   │   ├── crew_members.rs              # Crew member routes
│   │   ├── flights.rs                   # Flight routes
│   │   ├── flight_seats.rs              # Flight seat routes
│   │   ├── routes.rs                    # Route routes
│   │   ├── tickets.rs                   # Ticket routes
│   │   └── users.rs                     # User routes
│   └── utils/                           # Utility functions
│       ├── mod.rs                       # Utils module exports
│       ├── date_format.rs               # Date formatting utils
│       ├── logger.rs                    # Logging setup
│       └── validators.rs                # Input validation helpers
