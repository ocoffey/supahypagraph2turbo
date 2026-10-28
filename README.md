# American English SuperHypergraph Dictionary

The goal of this project is to create a SuperHyperGraph, which will define many
relationships and comparison means for the US English dictionary of words.

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
could be most forms of collection data structures! A named array is a HyperEdge,
where each element within is contained vertex. A HashMap is also a HyperEdge, where
each (key, value) pair is a bounded vertex.

### HyperGraph

A HyperGraph is a collection of HyperEdges! It can be thought of as one hierarchical
level above the HyperEdges within.

An example

## Particular Course Goals

### CSC 592 - Information Retrieval

Through the use of this superhypergraph, we should be able to match mistyped
words to their intended word with higher accuracy. This will make use of key
adjacency on the keyboard!

# Should we keep linear relationships for detecting idea deletion candidates?

- Pop structs against test by letter, and not by joint?
  - Incorporating the index later???
