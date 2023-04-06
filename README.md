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

The namespace is created with a POST request on `/namespace`:

```json
{
    "namespace": "<namespace>"
}
```


### Component Creation

The component is created with a POST request on `/<namespace>/component`:

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

The entities are created with a POST request on `/<namespace>/<component>`:

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

The entity state is updated with a POST request on `/<namespace>/<component>/entities`:

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
    }
]}
```