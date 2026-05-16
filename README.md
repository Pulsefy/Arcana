
# Arcana

**Unlock Your Financial Realm**

A futuristic personal finance and instant global payment platform built on the **Stellar blockchain**.

Arcana transforms your digital identity into a powerful **3D holographic financial profile** — combining wallet, credit reputation, payment terminal, and personal financial realm into one seamless experience.

---

## ✨ Overview

Arcana enables instant, borderless payments and financial interactions using beautiful AR-enhanced 3D profiles. Send money by pointing your camera, split bills in shared holographic spaces, and build your financial reputation through an on-chain **Arcana Score**.

Built as a **Turborepo** monorepo for maximum developer experience and scalability.

---

## 🚀 Key Features

- **Arcana Profile** — Living 3D/AR holographic financial identity
- **Realm Send** — Instant payments using profile names or AR scanning
- **AR Instant Pay** — Point-and-pay with immersive visual effects
- **Arcana Score** — Dynamic on-chain reputation and credit scoring system
- **Arcana Loans** — Borrow and lend using profile reputation as collateral
- **Multi-currency Support** — Seamless stablecoin conversions
- **Shared Realms** — Group expenses with 3D visualizations
- **Financial Fate** — AI-powered financial insights

---

## 🛠 Tech Stack

- **Blockchain**: Stellar Network + **Soroban** Smart Contracts (Rust)
- **Frontend**: Next.js 15 (App Router) + TypeScript + TailwindCSS
- **3D & AR**: React Three Fiber, Three.js, 8th Wall
- **Backend**: NestJS / Express
- **Monorepo**: Turborepo + pnpm
- **Database**: PostgreSQL + Prisma
- **State Management**: Zustand + TanStack Query

---

## 📁 Project Structure

```bash
arcana/
├── apps/
│   ├── web/                    # Next.js web application (frontend)
│   ├── mobile/                 # React Native / Expo app (future)
│   └── docs/                   # Documentation site
├── backend/                    # Backend services (NestJS / Express)
│   ├── src/
│   ├── prisma/
│   └── package.json
├── soroban/                    # Soroban smart contracts (Rust)
│   ├── src/                    # Soroban contract source code
│   ├── scripts/                # Deployment & testing scripts
│   ├── tests/                  # Contract tests
│   ├── Cargo.toml              # Soroban project configuration
│   └── package.json            # Turborepo integration
├── packages/
│   ├── ui/                     # Shared UI components + 3D components
│   ├── core/                   # Core business logic & utilities
│   ├── stellar/                # Stellar & Soroban integration layer
│   ├── types/                  # Shared TypeScript types
│   ├── hooks/                  # Shared React hooks
│   └── config/                 # Shared configs (ESLint, TS, Tailwind)
├── tools/                      # Internal tooling and scripts
├── turbo.json
├── pnpm-workspace.yaml
├── .env.example
└── README.md
```

---

## 🏁 Quick Start

### Prerequisites
- Node.js 20+
- pnpm 9+
- Rust (for Soroban contract development)
- Stellar CLI (`stellar-cli`)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourorg/arcana.git
cd arcana

# Install dependencies
pnpm install

# Copy environment variables
cp .env.example .env.local

# Start development servers
pnpm dev
```

### Available Scripts

```bash
pnpm dev          # Start all apps (web + backend)
pnpm build        # Build entire workspace
pnpm lint         # Lint all packages
pnpm test         # Run tests
pnpm soroban:build   # Build Soroban contracts
pnpm soroban:test    # Run Soroban contract tests
pnpm backend:dev     # Start only backend
```

---

## 🧩 Architecture

- **Frontend (`apps/web`)**: 3D/AR interface and user experience
- **Backend (`backend`)**: API layer, user management, off-chain logic, notifications
- **Soroban (`soroban`)**: On-chain logic including Arcana Profile, Arcana Score, Realm Send, and financial reputation system
- **Packages**: Reusable libraries shared across frontend, backend, and contracts

---

## 🌌 Roadmap

- **Phase 1 (MVP)**: Core payments, Arcana Profile, basic Arcana Score
- **Phase 2**: AR payments, Soroban-powered loans, Shared Realms
- **Phase 3**: Mobile app, AI insights, advanced reputation system
- **Phase 4**: Cross-chain support and institutional features

---

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guide](CONTRIBUTING.md) first.

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## 🔗 Links

- [Documentation](https://docs.arcana.finance)
- [Stellar Network](https://stellar.org)
- [Soroban Documentation](https://soroban.stellar.org)

---

**Made with passion for the future of finance.**

---

