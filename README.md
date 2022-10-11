1. cargo add diesel dotenv actix --features "diesel/postgres diesel/r2d2 diesel/chrono"
2. cargo install diesel_cli --no-default-features --features postgres
3. cargo install diesel_cli_ext
4. diesel setup
5. rm -r migrations
6. $PSDefaultParameterValues['Out-File:Encoding'] = 'utf8'
7. diesel print-schema > src/schema.rs
8. diesel_ext > src/db_models.rs
9. https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857