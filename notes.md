# Zero To Axum
A complement to the book Zero To Production by Luca Palmieri using Axum in lieu of Actix

## Table of Contents

## Chapter 1: Getting Started
(no code)

## Chapter 2: Building an Email Newsletter
(no code)

## Chapter 3: Sign Up a New Subscriber

### [IntoResponse](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)
Axum uses `IntoResponse` where Actix uses `HttpResponse`.
It allows you to return common types from your handlers without having to be explicit. Otherwise, you could return types that *already* `impl IntoResponse` or implemenent it for your own type.
Limitations:
- can only be used to return a single type
- can lead to type inference issues when used with `Result` and `?`


# ch 4 telemetry
## changed files:
  - /tests/health_check.rs
    * create our subscriber instance that is singular
  - startup.rs
    * tracing middleware for request_id
  [x] telemetry.rs
    * `get_subscriber` and `init_subscriber` functions
  [x] main.rs
    * get and init subscriber
  - add tracing events and instrument

    
## tracing
  - for handlers
  - for tests in `spawn_app` [p121]
    * poses a problem because it will init a subscriber multiple times. solution: `std::sync::Once`
      - author uses `once_cell` crate like a dickhead
    - know about 'test parallelism'

  - sqlx query gets `instrument(tracing_object)` called on it
  - `#[tracing::instrument()]`
  - you can `skip` inputs you want ignored
    - this is important not to log all args b/c that's what `insrument` does by default (Z2P uses `secrecy` crate)
  - `fields()`
  - `info_span!`

1. subscriber & tracing events in handler
2. `Once` for `spawn_app()`
3. middleware for `request_id`
4. `secrecy`

# ch 5 deployments
## build our docker image
## sqlx offline mode
- `sqlx prepare`
- set `SQLX_OFFLINE` env var to `true` in docker file so sqlx will look to metadata rather than try to find a running db
## port issues
- using `0.0.0.0` over `127.0.0.1`
  * the former as host will accept connections from any interface, whereas the latter just accepts from local
  * using zeroes for docker image and 127-1 for local dev
  * [ ] change config to use the right port under the proper circumstances. this is in config files and `main.rs`
  * [ ] shrink our docker image
    - exclude unnecessary files (`.dockerignore`)
  * [ ] builder & runtime stage docker commands
  * docker layer caching

# ch 6 rejections
- bm: tokio supports test case macro(?)
  ## changed files:
  - `tests/health_check.rs`
    * add test for empty fields [p165]
    * change test to `400` [p181]
  - `routes/subscriptions.rs`
    * name validation [p169]
    * change functions for new type
  - add new module, `domain.rs`
    * create `parse` for our new type, `SubscriberName(String)` that validates and produces an instance of it
    * ~add `inner_ref` to `SubscriberName` [p177]~
    * `impl AsRef<T>` for `SubscriberName` [p]
    * change `parse` to return a `Result` [p183]
  
