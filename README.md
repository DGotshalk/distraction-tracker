# Distraction Tracker

Distraction Tracker is a web application that helps you keep track of your daily distractions. It allows you to log distractions as they occur throughout the day, and provides analytics and insights to help you understand your habits and identify areas for improvement.

This version of Distraction Tracker uses a MySQL database to store data. It has been set up to run in a Docker container, which makes it easy to deploy and scale.

## Requirements

To run Distraction Tracker, you will need the following:

- <a href="https://www.rust-lang.org/tools/install"> Rust </a> 
- <a href="https://docs.docker.com/engine/install/"> Docker </a>(version 17.05 or higher)
- <a href="https://docs.docker.com/compose/install/"> Docker Compose </a> (version 1.20 or higher)

## Getting Started

To get started with Distraction Tracker running locally in staging in Docker, follow these steps:

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
		docker compose -f stg.docker-compose.yml up -d
		```

	   This will start a MySQL container and a web application container. The application will be accessible at `http://127.0.0.1:8080`.

3. To stop the application, run:
   ```
   docker compose -f stg.docker-compose.yml down
   ```

4. This is a staging application. The web app will be built and ran like it would be in production. To rebuild it after changes, simply rerun the docker commands.

## Non-Docker

1. Clone the repository:

   ```
   git clone https://github.com/DGotshalk/distraction-tracker.git
   ```

2. Run a development mysql container
	```
	docker compose -f dev.docker-compose.yml up -d
	```

4. Create a `.env` file in the root directory of the repository, with the following contents:

	```
	DATABASE_URL="mysql://dt-dev:dt_dev_pass@mysql-dev/dt_dev"
	```

5. Run the following
	```
	cargo run 
	```
## Usage

- **dev.docker-compose.yml**: used to run a local mysql instance that is required by sqlx when developing. the connection paramters for this container needs to match your .env file.
- **stg.docker-compose.yml**:  used to test to see if the application will work properly while both the app and the database are conainerized.
- **docker-compose.yml**: used to build the production application. If I push the image to docker hub, then I only need the docker file. 
- 
To use Distraction Tracker, open a web browser and navigate to `http://localhost:8080`. You will see the home page, which displays a list of distractions for the current day.

## License

Distraction Tracker is licensed under the MIT License. See the `LICENSE` file for more information.
