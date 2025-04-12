# tb8-rs

A Rust implementation of the tb8 TfL open data server using Axum.

## Features

- HTTP API for Transport for London data
- Built with Axum, a lightweight, fast web framework
- Implements key endpoints from the original Python tb8 project

## Endpoints

The server implements these core API endpoints:

- `/lines` - Get information about TfL lines (tube, bus, etc.)
- `/lines-by-station` - Get lines organized by station
- `/lines/:id` - Get information about a specific line
- `/lines-by-mode/:mode` - Get lines by mode (tube, bus, etc.)
- `/arrivals-by-lines` - Get arrival predictions for lines
- `/arrivals-by-station` - Get arrival predictions for a station
- `/disruption-by-modes` - Get service disruptions by mode
- `/stations` - Get station information
- `/station-points` - Get station geographic points
- `/platforms` - Get platform information

## Environment Variables

- `PORT` - The port to run the server on (default: 4000)
- `TFL_API_KEY_ID` - Your TfL API key ID
- `TFL_API_PRIMARY_ACCESS_KEY` - Your TfL API primary access key

## Running Locally

1. Install Rust: https://www.rust-lang.org/tools/install
2. Clone this repository
3. Set up environment variables:
   ```
   export TFL_API_KEY_ID=your_key_id
   export TFL_API_PRIMARY_ACCESS_KEY=your_access_key
   ```
4. Run the server:
   ```
   cargo run
   ```

## Deployment

The project includes configuration for deploying to Railway:

```
railway up
```

## Differences from Python tb8

This Rust implementation focuses on core functionality from the original Python tb8 project:

- Uses Axum instead of FastAPI
- Implements a subset of the original endpoints
- Doesn't include Polars DataFrame integration for complex data manipulation
- Uses simple in-memory data structures for some endpoints instead of loading from files