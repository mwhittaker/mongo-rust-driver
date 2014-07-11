# libbson #

This library provides functionality to convert between four types:

- `*bson_t` - The representation of a bson document used by the C driver.
- Rust BSON data structures - The rustic in memory data structures.
- JSON strings - Strings representing json entities.
- Rust Json data structures - The rustic in memory data structures.

If we imagine each data type as a vertex in a graph and we want to provide a
function to convert between any two types, we want to form a clique. To do so,
we need only implement enough conversion functions whose transitive closure
forms a clique. All other conversions follow trivially.
