updated README for the `distraction-tracker` repository on the `docker-sql` branch, assuming that it uses MySQL and listens on port 8080:

# Distraction Tracker

Distraction Tracker is a web application that helps you keep track of your daily distractions. It allows you to log distractions as they occur throughout the day, and provides analytics and insights to help you understand your habits and identify areas for improvement.

This version of Distraction Tracker uses a MySQL database to store data. It has been set up to run in a Docker container, which makes it easy to deploy and scale.

## Requirements

To run Distraction Tracker, you will need the following:

- (Rust)[https://www.rust-lang.org/tools/install] 
- (Docker)[https://docs.docker.com/engine/install/] (version 17.05 or higher)
- (Docker Compose)[https://docs.docker.com/compose/install/] (version 1.20 or higher)

## Getting Started

To get started with Distraction Tracker running locally, follow these steps:

1. Clone the repository:

   ```
   git clone https://github.com/DGotshalk/distraction-tracker.git
   ```

3. Create a `.env` file in the root directory of the repository, with the following contents:

   ```
   DATABASE_URL="mysql://dt-dev:dt_dev_pass@localhost:3306/dt_dev"
   ```

4. Run the application using Docker Compose:

   ```
   docker compose -f dev.docker-compose.yml up -d
   ```

   This will start a MySQL container and a web application container. The application will be accessible at `http://localhost:8080`.

5. To stop the application, run:

   ```
   docker compose -f dev.docker-compose.yml down
   ```

## Usage

To use Distraction Tracker, open a web browser and navigate to `http://localhost:8080`. You will see the home page, which displays a list of distractions for the current day.

## Contributing

If you would like to contribute to Distraction Tracker, you are welcome to submit a pull request. Please make sure to create a new branch for your changes and to follow the existing code style and conventions.

## License

Distraction Tracker is licensed under the MIT License. See the `LICENSE` file for more information.
