# State Maschine Database for Distributed Systems

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


## API

This section summarizes the API to interact with the `StateDB`.


### Namespace Creation

The namespace is created with a `POST` request on `/namespace`:

```json
{
    "namespace": "<namespace>"
}
```


### Component Creation

The component is created with a `POST` request on `/<namespace>/component`:

```json
{
    "component": "<component>",
    "states": [
        "created",
        "in-process",
        "completed",
    ],
    "transitions": [
        {
            "from": "created",
            "to": "in-process",
        },
        {
            "from": "in-process",
            "to": "completed",
        }
    ],
    "initial_state": "created",
    "meta_data": {
        "latitude": "double",
        "longitude": "double",
        "location": "string"
    }
}
```


### Entity Creation

The entities are created with a `POST` request on `/<namespace>/<component>`:

```json
{[
    {
        "entity_id": "<correlation_id1>",
        "meta_data": {
            "latitude": 10.0,
            "longitude": 20.0,
            "location": "location"
        }
    },
    {
        "entity_id": "<correlation_id2>"
    }
]}
```


### Entity State Update

The state of one or multiple entities is updated with a `POST` request on `/<namespace>/<component>/entities`:

```json
{[
    {
        "entity_id": "<correlation_id1>",
        "meta_data": {
            "latitude": 11.0,
            "longitude": 21.0,
            "location": "location"
        },
        "from_state": "created",
        "to_state": "in-processing",
        "compute_instance": "<instance-id>" 
    },
    {
        "entity_id": "<correlation_id2>",
        "from_state": "in-processing",
        "to_state": "completed",
        "compute_instance": "<instance-id>"
    }
]}
```

All state updates in one API call are performed in transactional mode such that all updates succeed or none.


### Entity Query

The entities are query with a `GET` request on `/<namespace>/<component>/query`:

```json
{
    "entity_id": "<glob_expression>",
    "state": "<state1>|<state2>|...|<stateN>",
    "metadata": "latitude > 0.4 and longitude > 10.0 and location == 'location'",
    "aggregation": "count()"
}
```

The query operation reads consistently from the database in the context of parallel state updates.


### Requirements

The following non-functional requirements shall be included in the database design:

- 1mio entities shall be (bulk-)created in below 1min
- state updates shall be performed in parallel from multiple thousand compute instances
- entity queries in a component of 100mio entities shall complete in below 10secs


## CLI

The `statedb` executable is the central UI to interact with the database.


### Creation of Database

A new database is created with:

```console
statedb create --working-dir <working_dir> --config <config_file>
```


### Launch of Database Service

The database service is launched with:

```console
statedb launch --working-dir <working_dir> --port <database_port>
```


### Interaction with Database

The database console may be launched with:

```console
statedb console --port <database_port> --user <user> --password <password>

statedb> create namespace <namespace_file>
statedb> create component <component_file>
statedb> create entities <entities_file>
statedb> update entities <entities_file>
statedb> query entities <query_file>
```