# Rush - Real-time, Onchain Quiz Game Powered by Magic Block 

> **A lightning-fast, fully on-chain quiz game powered by MagicBlock's Ephemeral Rollups**

Rush is a competitive real-time quiz game that demonstrates the power of MagicBlock's Ephemeral Rollups technology on Solana, delivering ultra-low latency gaming experiences with sub-50ms response times. Players compete in 4-player quiz sessions with real-time scoring, leaderboards, and instant prize distribution.

NOTE: This game is open source and is built for community and fun, contributions are encouraged. 

## 🎮 Game Overview

Rush showcases how Ephemeral Rollups can enable new consumer experiences by leveraging blockchain technology for fully on-chain games:

- **4-Player Competitive Quiz Sessions**: Real-time multiplayer gameplay
- **10 Questions per Game**: Fast-paced quiz format  
- **Instant Scoring & Leaderboards**: Real-time score updates during gameplay
- **Automated Prize Distribution**: Winners receive SOL rewards automatically
- **Zero-Fee Gameplay**: Minimal or zero-fee transactions during active gameplay

## 🚀 Technology Stack

### MagicBlock Ephemeral Rollups
Rush leverages MagicBlock's innovative Ephemeral Rollups architecture, which provides ultra-low latency (10ms block time), near-zero transaction fees, and horizontal scalability while maintaining full Solana compatibility.

**Key Benefits:**
- **⚡ Ultra-Low Latency**: Latencies as low as 50 milliseconds for real-time gaming experiences
- **💸 Cost Efficient**: Drastically reduced costs, enabling minimal or zero-fee transactions
- **🔗 Native Composability**: Build natively on Solana with no bridges, no fragmentation, no liquidity headaches
- **📈 Horizontal Scaling**: Seamlessly spinning up multiple ephemeral rollups to process millions of transactions per second

### Core Technologies
- **Solana Blockchain**: High-performance base layer
- **Anchor Framework**: Solana smart contract development
- **TypeScript/JavaScript**: Client-side development
- **Ephemeral Rollups SDK**: MagicBlock's rollup integration

## 🏗️ Architecture

### Game Flow

1. **Base Layer (Solana Mainnet)**
   - Program initialization
   - Player profile creation
   - Session creation with quiz questions
   - Prize vault management

2. **Delegation Phase**
   - Session and player accounts delegated to Ephemeral Rollups
   - Accounts are now delegated to the ER via a CPI call to the program but remain readable on base layer

3. **Ephemeral Rollup (Real-time Gameplay)**
   - Players join sessions (4 max per session)
   - Real-time answer submission
   - Live score calculation and leaderboard updates
   - Transactions processed with ultra-low latency

4. **Settlement (Back to Mainnet)**
   - Final scores and rankings calculated
   - Winners determined
   - State committed back to Solana
   - Automatic prize distribution

### Smart Contract Structure

```
programs/rush/src/
├── lib.rs                 # Program entry point with all instructions
├── state.rs              # All account structures and data models
├── constants.rs          # Program constants and PDA seeds
├── errors.rs            # Custom error definitions
└── contexts/            # Instruction implementations
    ├── init.rs          # Program initialization
    ├── profile.rs       # Player profile management
    ├── fund.rs          # Fund the Vault
    ├── create_session.rs # Quiz session creation
    ├── batch_delegate.rs # Batch Delegate all 4 playersession accounts to the rollup
    ├── delegate_session.rs # Delegate accounts to rollup
    ├── join_session.rs  # Player joins session
    ├── start_quiz.rs    # Begin gameplay
    ├── submit_answer.rs # Real-time answer submission
    ├── end_game.rs      # Game completion and settlement
    └── price.rs         # Prize distribution
```

## 🎯 Game Mechanics

### Session Creation
- Admin creates quiz session with 10 predefined questions
- Session includes question IDs and correct answers
- Prize pool funded from game vault

### Player Experience
1. **Profile Creation**: One-time player registration
2. **Session Joining**: Join available sessions (max 4 players)
3. **Real-time Gameplay**: Submit answers with instant feedback
4. **Live Leaderboards**: See ranking updates in real-time
5. **Instant Rewards**: Winners receive prizes automatically

### Scoring System
- **100 points** per correct answer
- **Real-time leaderboard updates** during gameplay
- **Final rankings** determine prize distribution

## 🛠️ Installation & Development

### Prerequisites
- [Rust](https://rustup.rs/) 1.75+
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) 1.18+
- [Anchor](https://book.anchor-lang.com/getting_started/installation.html) 0.31+
- [Node.js](https://nodejs.org/) 18+
- [Yarn](https://yarnpkg.com/)

### Setup

1. **Clone the repository**
```bash
git clone <repository-url>
cd rush
```

2. **Install dependencies**
```bash
yarn install
```

3. **Build the program**
```bash
anchor build
```

4. **Deploy to devnet**
```bash
anchor deploy --provider.cluster devnet
```

5. **Run tests**
```bash
anchor test
```

### Configuration

Update `Anchor.toml` with your:
- Solana RPC endpoint
- Wallet keypair path
- Program ID

## 📋 Usage Examples

### Initialize the Program
```javascript
await program.methods
  .initialize(
    new anchor.BN(1000000), // First prize (lamports)
    new anchor.BN(500000)   // Second prize (lamports)
  )
  .rpc();
```

### Create a Quiz Session
```javascript
const sessionId = new Uint8Array(32); // Generate unique session ID
const questionIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const correctAnswers = [0, 1, 2, 0, 3, 1, 0, 2, 1, 3];

await program.methods
  .createSession(Array.from(sessionId), questionIds, correctAnswers)
  .rpc();
```

### Join a Session
```javascript
await program.methods
  .joinGameSession(Array.from(sessionId))
  .rpc();
```

### Submit an Answer (In Ephemeral Rollup)
```javascript
await program.methods
  .submitAnswer(
    Array.from(sessionId),
    0, // question index
    1  // answer choice (0-3)
  )
  .rpc();
```

## 🏆 Game States & Flow

### Session States
- **Initialized**: Session created, waiting for players
- **Ready**: 4 players joined, ready to start
- **Live**: Game in progress on Ephemeral Rollup
- **Completed**: Game finished, scores finalized

### Account Delegation Flow
1. **Create Session** (Base Layer)
2. **Delegate Session Accounts** → Ephemeral Rollup
3. **Players Join** (Ephemeral Rollup)
4. **Delegate Player Accounts** → Ephemeral Rollup
5. **Real-time Gameplay** (Ephemeral Rollup)
6. **Commit & Undelegate** → Base Layer
7. **Distribute Prizes** (Base Layer)

## 🔧 Key Features

### Ephemeral Rollups Integration
- **Delegation Macros**: `#[delegate]` for account delegation
- **Commit Macros**: `#[commit]` for state settlement
- **Real-time Updates**: Live score and leaderboard changes
- **Gas-free Gameplay**: No transaction fees during active play

### Prize System
- **Automated Distribution**: Smart contract handles payouts
- **Vault Management**: Secure prize pool storage
- **Winner Calculation**: Final rankings determine rewards

### Security Features
- **Session Validation**: Only authorized players can join
- **Answer Verification**: Prevent duplicate submissions
- **State Integrity**: Ephemeral Rollup state validation
- **Fraud Protection**: Dynamic fraud proof window and light clients ensure security

## 📚 Understanding Ephemeral Rollups

Ephemeral Rollups leverage Solana's account structure and parallel execution to optimize state management. By structuring state into clusters, users can lock accounts and temporarily shift execution to a dedicated auxiliary layer.

### How It Works
1. **State Delegation**: Accounts are delegated to the Ephemeral Rollup via the Delegation Program (A CPI Call between your base layer program and the Ephemeral program to transfer ownership but ephemeral)
2. **Parallel Processing**: The SVM executes transactions concurrently, significantly boosting throughput and scalability
3. **Read Accessibility**: Even as accounts are delegated, transactions on the base layer can still read their state
4. **Settlement**: State changes are committed back to Solana after verification

## 🤝 Contributing

We welcome contributions! Please see our contributing guidelines and feel free to submit issues or pull requests.

### Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

## 📄 License

This project is licensed under the ISC License.

## 🔗 Resources

- [MagicBlock Documentation](https://docs.magicblock.gg/)
- [Ephemeral Rollups Whitepaper](https://arxiv.org/html/2311.02650v2)


*Rush demonstrates the future of fully on-chain gaming - where blockchain technology meets real-time user experiences without compromise.*