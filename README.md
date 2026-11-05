# American English SuperHypergraph Dictionary

The goal of this project is to create a 2-SuperHyperGraph (of naming scheme
n-SuperHyperGraph), which will define many relationships and comparison means
for the US English dictionary of words.

Each word within the dictionary is a SuperhyperGraph.

## Initial Definitions

### Vertex/Node

One of the two fundamental units upon which graphs are built. Vertices typically
represent some value, whether it be a number, letter, or more complex combination.
Vertices are connected to each other by Edges

### Edge

The other fundamental unit which builds graphs. Edges connect vertices together,
and can be either directed or undirected.

Suggesting we have a graph with each
node representing an event, a directed each between nodes would represent the sequence
of these events! By contrast, an undirected edge running between nodes would
represent those nodes having some connection, with the form of connection being
less known to us.

### HyperEdge

An edge that **simultaneously** connects multiple vertices. Since the connection
between all vertices is simultaneous and continuous, a HyperEdge can be thought
to "bound" these vertices!

The pure definition doesn't always make easy sense. An example of a HyperEdge
could be most forms of collection data structures! A named `Vec` is a HyperEdge,
where each element within is contained vertex.

### HyperGraph

A HyperGraph is a collection of HyperEdges! It can be thought of as one hierarchical
level above the HyperEdges within.

An example of a HyperGraph would be a `HashMap`, with each `key` as a HyperEdge,
and each `value` being the vertex/vertices bounded by that HyperEdge! So if we had
`String`s as `key`s, each `value` could be a `Vec<some item>`.

### SuperHyperGraph

One of the neat qualities between HyperGraphs and HyperEdges is that a singular
representation of data can have multiple interpretations from a singular representation!

As an example of this, take a `Vec<char>`. If we choose to ignore the implicit ordering
and simply use this structure as an efficient container of `char`, then it would
represent a hyperedge!

But if we _do_ relationally order elements within this `Vec<char>`, then we could
consider the `Vec` to be a hypergraph, each index as a hyperedge, and each char
at a corresponding index would be set of elements the hyperedge bounds (in this
case, each set would only contain 1 `char`! Which is a valid quality of hyperedges).

### n-SuperHyperGraph

This is the generalization of hierarchical ordering past a SuperHyperGraph.
Akin to a k-d tree, where 'k' represents a specific number of dimensions, the 'n'
within this definition corresponds to the amount of hierarchical SuperHyperGraphs
that exist!

The reason why our dictionary will be a 2-SuperHyperGraph is the following structure:

- Each word contained within the dictionary is a SuperHyperGraph (2)
  - Each word itself is a SuperHyperGraph (1)
    - The mapping of letters -> joints, letter ordinals -> joints,
      and indices -> joints are each HyperGraphs
      - Each unordered set of these joints represents a HyperEdge

If we decided to create this structure for dictionaries from other languages, then
if would take the form of a 3-SuperHyperGraph, where the highest order SuperHyperGraph
have entries for each language!

## Particular Course Goals

### CSC 592 - Information Retrieval

Through the use of this 2-superhypergraph, we should be able to match mistyped
words to their intended word with higher accuracy. This will make use of key
adjacency on the keyboard!

### CSC 546 - Algorithms for Big Data

The purpose of this project within the scope of this course is in trying to be _efficient_.
There are easy ways to represent these relations, but they contain much redundant
information, can easily hog a lot of space, and can potentially take a lot of time
to perform lookups.
