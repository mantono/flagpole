# flagpole
Flagpole is a minimal and simple service for hosting [feature flagging](https://featureflags.io/feature-flag-introduction) configurations.

## Project Priorities
#### Simple
The API - and code - should be intuitive and unsurprising. _Less is more_ is a guiding principle.

#### Resource Efficient
Running the service should require minimal resources, both in terms of memory and CPU usage.

#### Secure & Reliable
This project should be built in such way that it has as few attack vectors as possible. Reducing complexity is a corner stone for achieving this.

#### Performant
Performance is important and desirable, but not as important as the above priorities.

## Building & Installing
If you have [cargo](https://doc.rust-lang.org/cargo) installed, run `cargo install --path .` in the root of this repository.

## Usage
Launch the application simply by typing `flagpole` in a terminal.

See `flagpole --help` for command line flags, such as configuring API key or port number.

### Authorization
Authorization of requests can optionally be enabled, in which case all requests that _alters_ state (`PUT` and `DELETE`) requires authorization,
while other requests (`HEAD` and `GET`) does not require authorization. Authorization can be omitted if this service runs in a context where it
is not needed.

### API
See [API](API/) directory for documentation about the different API requests and examples.

## Optional Features
| Feature   | Enabled by Default | Comment |
| :-------: | :---------------:  | :------ |   
| `logging` | true               | Add support for logging |
| `redis`   | false              | Persist feature flags to Redis |
