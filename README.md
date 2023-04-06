# StateDB: State Maschine Database for Distributed Systems

Distributed systems consist of multiple logical stages running on multiple machines.
Data is split into entities whose state is modified while flowing through the system.
The union of the entities' states in the logical stages comprises the overall state of the distributed system.

This project `StateDB` implements a scalable database to manage the state of entities in a distributed system.

## Concepts

The `StateDB` supports multi-tenancy by storing multiple system instances in **namespaces** (e.g. for development use cases).
Within each namespace, a collection of **components** stores a set of **state machines** for the **entities** of the system.
Each entity is identified by a **correlation id**.
The state machine consists of a set of **states** and **transitions** between the states.
Each entity is in exactly one state.
The transitions from one state to another are stored in **state transitions**.