# Setu API

A simple solid color image generation API service written in Rust.

## Features

- Generate solid color images using hex color codes
- Customizable image dimensions
- Support for RGBA color channels (including transparency)
- PNG format output
- RESTful API interface
- Lightweight and fast

## Tech Stack

- Rust 2024 Edition
- actix-web (Web framework)
- image (Image processing library)
- serde (Serialization/Deserialization)

## Requirements

- Rust 2024 Edition
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/0x002500/setu-api.git
cd setu-api
```

2. Build the project:
```bash
cargo build --release
```

3. Run the service:
```bash
cargo run --release
```

The service will start on `http://127.0.0.1:8080`

## API Usage

### Endpoints

#### 1. Health Check
```
GET /
```
Response: "Thanks for using setu-api!"

#### 2. Generate Image
```
GET /pic?color={hex_color}&width={width}&height={height}
```

Parameters:
- `color`: Hex color code (6 or 8 digits for RGBA)
- `width`: Image width in pixels
- `height`: Image height in pixels

Response: PNG image file

### Examples

1. Generate a red 200x200 pixel image:
```
GET http://127.0.0.1:8080/pic?color=FF0000&width=200&height=200
```

2. Generate a semi-transparent blue 100x100 pixel image:
```
GET http://127.0.0.1:8080/pic?color=0000FF80&width=100&height=100
```

## Development

To contribute to the development, make sure you have the Rust development environment installed.

1. Install dependencies:
```bash
cargo build
```

2. Run tests:
```bash
cargo test
```

## Dependencies

- actix-web: ^4.11.0
- image: ^0.25.6
- serde: ^1.0.219
- serde_json: ^1.0.142

## License

MIT License

## Contributing

Issues and Pull Requests are welcome!

## Author

- 0x002500
