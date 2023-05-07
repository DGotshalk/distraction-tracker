# Distraction Tracker

Distraction Tracker is a web application that helps you keep track of your daily distractions. It allows you to log distractions as they occur throughout the day, and provides analytics and insights to help you understand your habits and identify areas for improvement.

This version of Distraction Tracker uses a MySQL database to store data. It has been set up to run in a Docker container, which makes it easy to deploy and scale.

## Requirements

To run Distraction Tracker, you will need the following:

- <a href="https://www.rust-lang.org/tools/install"> Rust </a> 
- <a href="https://docs.docker.com/engine/install/"> Docker </a>(version 17.05 or higher)
- <a href="https://docs.docker.com/compose/install/"> Docker Compose </a> (version 1.20 or higher)

## Getting Started

To get started with Distraction Tracker running locally in development in Docker, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/DGotshalk/distraction-tracker.git
   ```

2. Run the application using Docker Compose:
	1. First run:
		```
		cargo build
		```
	2. Then run:
	
		```
		docker compose -f dev.docker-compose.yml up -d
		```

	   This will start a MySQL container and a web application container. The application will be accessible at `http://127.0.0.1:8080`.

3. To stop the application, run:

   ```
   docker compose -f dev.docker-compose.yml down
   ```

4. This is a development application. The web app is linked to the directory on the host filesystem where the source code is. When you make changes, simply run `cargo build` and rerun the docker commands.

## Non-Docker

1. Clone the repository:

   ```
   git clone https://github.com/DGotshalk/distraction-tracker.git
   ```

2. Install mysql or have an instance running locally or remotely, your choice

3. Create a `.env` file in the root directory of the repository, with the following contents:

   ```
   DATABASE_URL="mysql://user:pass@host:port/database"
   ```

4. Run the following
	```
	cargo run 
	```

## Usage

To use Distraction Tracker, open a web browser and navigate to `http://localhost:8080`. You will see the home page, which displays a list of distractions for the current day.

## License

Distraction Tracker is licensed under the MIT License. See the `LICENSE` file for more information.
