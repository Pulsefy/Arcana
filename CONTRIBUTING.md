# Contributing to Arcana

First off, thank you for considering contributing to Arcana! We're building a futuristic personal finance and instant global payment platform, and we welcome your help to make it better.

## Branching Strategy

We follow a structured branching strategy to maintain a clean history and simplify our development process across the monorepo. Please create a new branch for every feature or bug fix:

- **`main`**: The stable branch. All production-ready code lives here.
- **`feature/<feature-name>`**: Used for developing new features (e.g., `feature/ar-payments-ui`).
- **`fix/<bug-name>`**: Used for bug fixes (e.g., `fix/stellar-wallet-connection`).
- **`docs/<doc-name>`**: Used for documentation updates.
- **`chore/<task-name>`**: Used for maintenance tasks, dependency updates, or monorepo tooling changes.

Always branch off from `main` and create a Pull Request back into `main` when your work is complete.

## Local Environment Setup

Arcana is built as a Turborepo monorepo. To get your local development environment up and running, follow these steps.

### Prerequisites

Ensure you have the following installed:
- **Node.js**: Version 20+
- **Package Manager**: pnpm version 9+
- **Rust**: Required for Soroban contract development
- **Stellar CLI**: (`stellar-cli`)

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Pulsefy/Arcana.git
   cd arcana
   ```

2. **Set up Environment Variables**:
   Copy the example environment file and fill in the necessary values:
   ```bash
   cp .env.example .env.local
   ```

3. **Install Dependencies**:
   Install the required workspace dependencies using `pnpm`:
   ```bash
   pnpm install
   ```

4. **Run the Development Server**:
   You can start all apps (web frontend + backend) simultaneously using Turborepo:
   ```bash
   pnpm dev
   ```

### Additional Available Scripts

- `pnpm build`: Build the entire workspace
- `pnpm lint`: Lint all packages
- `pnpm test`: Run tests
- `pnpm soroban:build`: Build Soroban smart contracts
- `pnpm backend:dev`: Start only the backend

## Pull Request Process

1. Ensure any changes or new features are fully tested.
2. Run the linter across the workspace to ensure code style consistency: `pnpm lint`.
3. If you add new features that require configuration changes, make sure to update the `README.md` or relevant documentation in the `apps/docs` app.
4. Create a descriptive Pull Request with a summary of the changes, linking any relevant issues.
