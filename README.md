# ctt client


## Dev
- schema.grahql is downloaded from the server (`/schema`)
  - see `async_graphql::Schema::sdl`
- query.graphql is manually written, with help from graphiQL 
- these are used to generate queries.rs
  - `cargo graphql-client generate -s schema.graphql query.graphql`
    - this is then manually edited (to add clap derives) and copied into src
