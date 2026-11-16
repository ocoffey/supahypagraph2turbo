# American English Adjacency Dictionary

The goal of this project is to augment word autocomplete functionality by making
use of information about _letter adjacency_ from a US English keyboard layout!

As users type letters into the keyboard, we will perform progressive lookups
within a trie structure, while generating additional fuzzy lookups based upon
each adjacent letter that a user types!

This is a return to the originally proposed problem space.

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

In the case of this program, we use a `HashSet` to represent elements bounded
by a HyperEdge. This is because a `HashSet` will non-relationally bound each
element (each element has O(1) lookup speed, and lookups do not require relation
to other elements within the `HashSet`), whereas a `Vec` holds some form of
relation due to its indexed nature.

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

## Particular Course Goals

### CSC 592 - Information Retrieval

Through the use of this Adjacency HyperGraph, we modify word autocomplete
functionality with slight semantic autocorrection with more information from our
input domain.

In considering adjacent letters on a US English keyboard layout, we
widen the scope of accepted inputs for user words based on the real-world interaction
potential of users mistakenly typing a letter with high proximity.

### CSC 546 - Algorithms for Big Data

The purpose of this project within the scope of this course is in trying to be _efficient_.

Typically, an autocomplete functionality takes a wide input space, and narrows
its scope with each progressive step. By using adjacency hyperedges at each step,
we are instead progressively _adding_ lookup branches at each progressive step.

With this being the case, the program should still maintain reasonable limits, based
on the fact that many hyperedge subtrees will be non-starters. As an example, a
user entering the letter `A` generates potentials for the letters `'Q', 'W', 'S',
'Z'`, but the letter `Z` doesn't occur often in the US English dictionary, so in
most cases it would fail to generate a corresponding lookup.
