# Wiki-Trust

## Overview
Wiki-Trust is an advanced research tool designed to analyze and track the evolution of Wikipedia articles, focusing on understanding potential political biases in their revisions. This tool extracts revisions of Wikipedia articles, identifies contributors, and reconstructs article versions at different points in time. The ultimate goal is to develop a web dashboard that allows users to visually explore these revisions, understand contribution patterns, and analyze potential biases of contributors.

## Features
- **Revision Tracking**: Extracts and tracks Wikipedia article revisions over time.
- **Contributor Identification**: Identifies users who have made contributions to each article.
- **Article Reconstruction**: Reconstructs the state of an article at any given point in its history.
- **Change Attribution ("Git Blame")**: Determines which user contributed specific parts of an article.
- **Bias Analysis**: Aims to identify potential political biases in contributions.
- **Web Dashboard**: Provides a user-friendly interface for exploring article histories and analyses.

## Installation

### Prerequisites
- Rust Programming Language
- Diesel ORM
- PostgreSQL or SQLite
- Reqwest Library for Rust

### Setup
1. **Clone the Repository**: `git clone https://github.com/yourusername/Wiki-Trust.git`
2. **Database Setup**: Configure your database settings in `Diesel.toml`.
3. **Build the Project**: Run `cargo build` in the project directory.
4. **Run Migrations**: Execute `diesel migration run` to set up your database schema.

## Usage
- **Starting the Tracker**: Run `cargo run` to start extracting revisions.
- **Web Dashboard**: Access the dashboard at `http://localhost:8000` (default) to explore the data.

## Ethical Considerations and Privacy
This project is committed to ethical research practices and respects the privacy of Wikipedia contributors. Our methodology for bias analysis is transparent, and we ensure that the data used does not infringe upon the privacy rights of individuals. We advise users of Wiki-Trust to adhere to these principles and use the tool responsibly.

## Contributing
We welcome contributions from developers, researchers, and enthusiasts. Please read `CONTRIBUTING.md` for guidelines on how to submit contributions.

## License
This project is licensed under the MIT License - see the `LICENSE` file for details.

## Acknowledgments
- Wikipedia and the Wikimedia Foundation for providing open access to article revision data.
- Contributors to the Rust programming language and its ecosystem.

## Disclaimer
This tool is for research purposes only. The creators of Wiki-Trust are not responsible for any misuse of the tool or interpretations of the data provided.
