## Design

[graph-types]() divides the graph into adjacency storage engine and entry storage engine.

### Adjacency Storage Engine

The core property of a graph is nodes and edges. 

There are many different ways to represent a graph. 

Such as adjacency matrix, adjacency list, adjacency set, etc.

The most common one is adjacency list.

### Entry Storage Engine

Other properties of the graph are stored in the entry storage engine.

For example weights, labels, colors, etc.

Each of the entry can be stored in different data structures.

For example, weights can be stored in a vector, labels can be stored in a hash map, some complex structures can even be
stored in disk or a database.

