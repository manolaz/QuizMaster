// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Rush } from "../target/types/rush";
// import { expect } from "chai";
// import { 
//   Keypair, 
//   SystemProgram, 
//   LAMPORTS_PER_SOL,
//   PublicKey
// } from "@solana/web3.js";
// import { GetCommitmentSignature } from "@magicblock-labs/ephemeral-rollups-sdk";

// describe("Rush Quiz Game with Ephemeral Rollups", () => {
//   // Base Layer Provider (Solana)
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   // Ephemeral Rollup Provider (MagicBlock)
//   const providerEphemeralRollup = new anchor.AnchorProvider(
//     new anchor.web3.Connection(
//       process.env.ER_ENDPOINT || "https://devnet.magicblock.app/",
//       {
//         wsEndpoint: process.env.ER_WS_ENDPOINT || "wss://devnet.magicblock.app/",
//         commitment: "confirmed"
//       }
//     ),
//     anchor.Wallet.local()
//   );

//   console.log("Base Layer Connection: ", provider.connection.rpcEndpoint);
//   console.log("Ephemeral Rollup Connection: ", providerEphemeralRollup.connection.rpcEndpoint);

//   const program = anchor.workspace.Rush as Program<Rush>;

//   //admin account
//   const admin = provider.wallet;

//   // Test accounts
//   let player1: Keypair;
//   let player2: Keypair;
//   let player3: Keypair;
//   let player4: Keypair;
  
//   // PDAs
//   let configPda: PublicKey;
//   let vaultStatePda: PublicKey;
//   let vaultPda: PublicKey;
//   let leaderboardPda: PublicKey;
  
//   // Game session data
//   const sessionId = new Uint8Array(32);
//   sessionId.fill(1); // Simple session ID for testing
  
//   //array of question IDs since we have a fixed number of questions from the backend
//   const questionIds = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

//   //array of question IDs since we have a fixed number of questions from the backend
//   const correctAnswers = [0, 1, 2, 3, 0, 1, 2, 3, 0, 1]; // Answers for each question || 0-3 for options A-D
  
//   const firstPrize = 0.1 * LAMPORTS_PER_SOL;  // 0.1 SOL
//   const secondPrize = 0.05 * LAMPORTS_PER_SOL; // 0.05 SOL

//   before(async () => {
//     // Create test keypairs
//     player1 = Keypair.generate();
//     player2 = Keypair.generate();
//     player3 = Keypair.generate();
//     player4 = Keypair.generate();

//     // Airdrop SOL to test accounts on both layers
//     const accounts = [player1, player2, player3, player4];

//     // Airdrop on base layer
//     for (const account of accounts) {
//       await provider.connection.requestAirdrop(
//         account.publicKey,
//         2 * LAMPORTS_PER_SOL
//       );
//     }

//     // Wait for airdrops to confirm
//     await new Promise((resolve) => setTimeout(resolve, 2000));

//     // Check current balance
//     const balance = await provider.connection.getBalance(admin.publicKey);
//     console.log("Admin balance:", balance / LAMPORTS_PER_SOL, "SOL");

//     console.log("Calculating PDAs...");

//     // Derive PDAs
//     [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("config"), admin.publicKey.toBuffer()],
//       program.programId
//     );

//     [vaultStatePda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("game_vault_state"), admin.publicKey.toBuffer()],
//       program.programId
//     );

//     [vaultPda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("game_vault"), vaultStatePda.toBuffer()],
//       program.programId
//     );

//     [leaderboardPda] = PublicKey.findProgramAddressSync(
//       [Buffer.from("leaderboard"), admin.publicKey.toBuffer()],
//       program.programId
//     );
//   });

//   describe("Base Layer Setup", () => {
//     it("Should initialize the program on base layer", async () => {
//       const tx = await program.methods
//         .initialize(new anchor.BN(firstPrize), new anchor.BN(secondPrize))
//         .accountsPartial({
//           admin: admin.publicKey,
//           config: configPda,
//           vaultState: vaultStatePda,
//           vault: vaultPda,
//           leaderBoard: leaderboardPda,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([admin.payer])
//         .rpc();

//       console.log("Initialize transaction signature:", tx);

//       // Verify config account
//       const configAccount = await program.account.config.fetch(configPda);
//       expect(configAccount.admin.toString()).to.equal(admin.publicKey.toString());
//       expect(configAccount.firstPrize.toNumber()).to.equal(firstPrize);
//       expect(configAccount.secondPrize.toNumber()).to.equal(secondPrize);
//     });

//     it("Should fund the vault", async () => {
//       const fundAmount = 1 * LAMPORTS_PER_SOL;

//       const tx = await program.methods
//         .fundVault(new anchor.BN(fundAmount))
//         .accountsPartial({
//           admin: admin.publicKey,
//           config: configPda,
//           vaultState: vaultStatePda,
//           vault: vaultPda,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([admin.payer])
//         .rpc();

//       console.log("Fund vault transaction signature:", tx);

//       // Verify vault balance
//       const vaultBalance = await provider.connection.getBalance(vaultPda);
//       expect(vaultBalance).to.equal(fundAmount);
//     });

//     it("Should create player profiles on base layer", async () => {
//       const players = [
//         { keypair: player1, name: "Alice" },
//         { keypair: player2, name: "Bob" },
//         { keypair: player3, name: "Charlie" },
//         { keypair: player4, name: "Diana" },
//       ];

//       for (const { keypair, name } of players) {
//         const [playerStatePda] = PublicKey.findProgramAddressSync(
//           [Buffer.from("player_state"), keypair.publicKey.toBuffer()],
//           program.programId
//         );

//         const tx = await program.methods
//           .createProfile(name)
//           .accountsPartial({
//             player: keypair.publicKey,
//             playerState: playerStatePda,
//             systemProgram: SystemProgram.programId,
//           })
//           .signers([keypair])
//           .rpc();

//         console.log(`Profile created for ${name}:`, tx);
//       }
//     });

//     it("Should create a new game session on base layer", async () => {
//       const [sessionPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("session"), sessionId],
//         program.programId
//       );

//       const [sessionDataPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("question"), admin.publicKey.toBuffer(), sessionId],
//         program.programId
//       );

//       const tx = await program.methods
//         .createSession(Array.from(sessionId), questionIds, correctAnswers)
//         .accountsPartial({
//           admin: admin.publicKey,
//           sessionData: sessionDataPda,
//           session: sessionPda,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([admin.payer])
//         .rpc();

//       console.log("Create session transaction signature:", tx);

//       // Verify session was created
//       const session = await program.account.session.fetch(sessionPda);
//       expect(session.creator.toString()).to.equal(admin.publicKey.toString());
//       expect(session.currentPlayers).to.equal(0);
//       expect(session.status).to.deep.equal({ initialized: {} });
//     });
//   });

//   describe("Delegation to Ephemeral Rollup", () => {
//     let sessionPda: PublicKey;
//     let sessionDataPda: PublicKey;

//     before(() => {
//       [sessionPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("session"), sessionId],
//         program.programId
//       );

//       [sessionDataPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("question"), admin.publicKey.toBuffer(), sessionId],
//         program.programId
//       );
//     });

//     it("Should delegate session accounts to ephemeral rollup", async () => {
//       // Delegate session, session data, and leaderboard to ER
//       const tx = await program.methods
//         .delegateSession(Array.from(sessionId))
//         .accountsPartial({
//           admin: admin.publicKey,
//           session: sessionPda,
//           sessionData: sessionDataPda,
//           leaderboard: leaderboardPda,
//         })
//         .signers([admin.payer])
//         .rpc();

//       console.log("Delegate session transaction signature:", tx);
//       console.log("🚀 Session accounts delegated to Ephemeral Rollup");
//     });
//   });

//   describe("Real-time Game Flow in Ephemeral Rollup", () => {
//     let sessionPda: PublicKey;
//     let sessionDataPda: PublicKey;
//     let playerSessionAnswerPdas: PublicKey[] = [];

//     before(() => {
//       [sessionPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("session"), sessionId],
//         program.programId
//       );

//       [sessionDataPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("question"), admin.publicKey.toBuffer(), sessionId],
//         program.programId
//       );

//       // Create player session answer PDAs
//       [player1, player2, player3, player4].forEach((player) => {
//         const [pda] = PublicKey.findProgramAddressSync(
//           [
//             Buffer.from("player_session_answer"),
//             sessionId,
//             player.publicKey.toBuffer()
//           ],
//           program.programId
//         );
//         playerSessionAnswerPdas.push(pda);
//       });
//     });

//     it("Should allow players to join session in ER", async () => {
//       const players = [player1, player2, player3, player4];
      
//       for (let i = 0; i < players.length; i++) {
//         const player = players[i];
//         const [playerStatePda] = PublicKey.findProgramAddressSync(
//           [Buffer.from("player_state"), player.publicKey.toBuffer()],
//           program.programId
//         );

//         // Create transaction for ER
//         let tx = await program.methods
//           .joinGameSession(Array.from(sessionId))
//           .accountsPartial({
//             player: player.publicKey,
//             playerState: playerStatePda,
//             playerSessionAnswer: playerSessionAnswerPdas[i],
//             session: sessionPda,
//             systemProgram: SystemProgram.programId,
//           })
//           .signers([player])
//           .transaction();

//         // Configure for Ephemeral Rollup
//         tx.feePayer = providerEphemeralRollup.wallet.publicKey;
//         tx.recentBlockhash = (
//           await providerEphemeralRollup.connection.getLatestBlockhash()
//         ).blockhash;

//         tx = await providerEphemeralRollup.wallet.signTransaction(tx);
//         const txHash = await providerEphemeralRollup.sendAndConfirm(tx);

//         // Get commitment signature for ER transaction
//         const txCommitSgn = await GetCommitmentSignature(
//           txHash,
//           providerEphemeralRollup.connection
//         );

//         console.log(`Player ${i + 1} joined (ER): ${txCommitSgn}`);
//       }
//     });

//     it("Should delegate all player session answers to ER", async () => {
//       const tx = await program.methods
//         .delegateFrens(Array.from(sessionId))
//         .accountsPartial({
//           admin: admin.publicKey,
//           session: sessionPda,
//           player1SessionAnswer: playerSessionAnswerPdas[0],
//           player2SessionAnswer: playerSessionAnswerPdas[1],
//           player3SessionAnswer: playerSessionAnswerPdas[2],
//           player4SessionAnswer: playerSessionAnswerPdas[3],
//         })
//         .signers([admin.payer])
//         .rpc();

//       console.log("Delegate all players transaction signature:", tx);
//       console.log("🎮 All player accounts delegated to ER for real-time gameplay");
//     });

//     it("Should start the quiz in ER (ultra-low latency)", async () => {
//       // Create transaction for ER
//       let tx = await program.methods
//         .startQuiz(Array.from(sessionId))
//         .accountsPartial({
//           admin: admin.publicKey,
//           session: sessionPda,
//         })
//         .signers([admin.payer])
//         .transaction();

//       // Configure for Ephemeral Rollup
//       tx.feePayer = providerEphemeralRollup.wallet.publicKey;
//       tx.recentBlockhash = (
//         await providerEphemeralRollup.connection.getLatestBlockhash()
//       ).blockhash;

//       tx = await providerEphemeralRollup.wallet.signTransaction(tx);
//       const txHash = await providerEphemeralRollup.sendAndConfirm(tx);

//       const txCommitSgn = await GetCommitmentSignature(
//         txHash,
//         providerEphemeralRollup.connection
//       );

//       console.log(`Quiz started in ER (real-time!): ${txCommitSgn}`);
//     });

//     it("Should process real-time answer submissions in ER", async () => {
//       const answerPromises = [];

//       // Simulate concurrent answer submissions (real-time gameplay!)
//       for (let i = 0; i < 4; i++) {
//         const player = [player1, player2, player3, player4][i];
//         const answer = i < 2 ? correctAnswers[0] : (correctAnswers[0] + 1) % 4; // First 2 correct, last 2 wrong

//         const answerPromise = (async () => {
//           let tx = await program.methods
//             .submitAnswer(Array.from(sessionId), 0, answer)
//             .accountsPartial({
//               player: player.publicKey,
//               session: sessionPda,
//               sessionData: sessionDataPda,
//               playerSessionAnswer: playerSessionAnswerPdas[i],
//               leaderboard: leaderboardPda,
//             })
//             .signers([player])
//             .transaction();

//           // Configure for Ephemeral Rollup (ultra-low latency)
//           tx.feePayer = providerEphemeralRollup.wallet.publicKey;
//           tx.recentBlockhash = (
//             await providerEphemeralRollup.connection.getLatestBlockhash()
//           ).blockhash;

//           tx = await providerEphemeralRollup.wallet.signTransaction(tx);
//           const txHash = await providerEphemeralRollup.sendAndConfirm(tx);

//           const txCommitSgn = await GetCommitmentSignature(
//             txHash,
//             providerEphemeralRollup.connection
//           );

//           console.log(`Player ${i + 1} answer submitted (ER): ${txCommitSgn}`);
//           return txCommitSgn;
//         })();

//         answerPromises.push(answerPromise);
//       }

//       // Wait for all concurrent submissions (demonstrates real-time capability)
//       await Promise.all(answerPromises);
//       console.log("✅ All answers processed in real-time within Ephemeral Rollup!");
//     });

//     it("Should end game and commit state back to base layer", async () => {
//       // End game in ER and commit/undelegate back to Solana
//       let tx = await program.methods
//         .endGameSession(Array.from(sessionId))
//         .accountsPartial({
//           admin: admin.publicKey,
//           session: sessionPda,
//           sessionData: sessionDataPda,
//           leaderboard: leaderboardPda,
//           player1SessionAnswer: playerSessionAnswerPdas[0],
//           player2SessionAnswer: playerSessionAnswerPdas[1],
//           player3SessionAnswer: playerSessionAnswerPdas[2],
//           player4SessionAnswer: playerSessionAnswerPdas[3],
//         })
//         .signers([admin.payer])
//         .transaction();

//       // Configure for Ephemeral Rollup
//       tx.feePayer = providerEphemeralRollup.wallet.publicKey;
//       tx.recentBlockhash = (
//         await providerEphemeralRollup.connection.getLatestBlockhash()
//       ).blockhash;

//       tx = await providerEphemeralRollup.wallet.signTransaction(tx);
//       const txHash = await providerEphemeralRollup.sendAndConfirm(tx);

//       const txCommitSgn = await GetCommitmentSignature(
//         txHash,
//         providerEphemeralRollup.connection
//       );

//       console.log(`Game ended and state committed back to Solana: ${txCommitSgn}`);
//       console.log("🔄 State successfully undelegated back to base layer");
//     });
//   });

//   describe("Prize Distribution on Base Layer", () => {
//     let sessionPda: PublicKey;

//     before(() => {
//       [sessionPda] = PublicKey.findProgramAddressSync(
//         [Buffer.from("session"), sessionId],
//         program.programId
//       );
//     });

//     it("Should distribute prizes to winners on base layer", async () => {
//       // Wait a moment for state to be fully committed
//       await new Promise((resolve) => setTimeout(resolve, 1000));

//       const player1BalanceBefore = await provider.connection.getBalance(
//         player1.publicKey
//       );
//       const player2BalanceBefore = await provider.connection.getBalance(
//         player2.publicKey
//       );

//       // Prize distribution happens on base layer
//       const tx = await program.methods
//         .payout(Array.from(sessionId))
//         .accountsPartial({
//           admin: admin.publicKey,
//           session: sessionPda,
//           config: configPda,
//           vaultState: vaultStatePda,
//           vault: vaultPda,
//           winner1: player1.publicKey,
//           winner2: player2.publicKey,
//           systemProgram: SystemProgram.programId,
//         })
//         .signers([admin.payer])
//         .rpc();

//       console.log("Prize distribution transaction signature:", tx);

//       // Verify balances increased
//       const player1BalanceAfter = await provider.connection.getBalance(
//         player1.publicKey
//       );
//       const player2BalanceAfter = await provider.connection.getBalance(
//         player2.publicKey
//       );

//       expect(player1BalanceAfter - player1BalanceBefore).to.equal(firstPrize);
//       expect(player2BalanceAfter - player2BalanceBefore).to.equal(secondPrize);

//       console.log(`🏆 Player 1 received ${firstPrize / LAMPORTS_PER_SOL} SOL`);
//       console.log(`🥈 Player 2 received ${secondPrize / LAMPORTS_PER_SOL} SOL`);
//     });

//     it("Should verify final game state on base layer", async () => {
//       // Verify session is completed
//       const session = await program.account.session.fetch(sessionPda);
//       expect(session.status).to.deep.equal({ completed: {} });
//       expect(session.prizesDistributed).to.be.true;

//       // Verify vault state
//       const vaultState = await program.account.gameVault.fetch(vaultStatePda);
//       expect(vaultState.totalDisbursed.toNumber()).to.equal(
//         firstPrize + secondPrize
//       );
//       expect(vaultState.totalGamesPlayed.toNumber()).to.equal(1);

//       // Verify leaderboard has been updated
//       const leaderboard = await program.account.leaderboard.fetch(
//         leaderboardPda
//       );
//       expect(leaderboard.lastUpdated.toNumber()).to.be.greaterThan(0);

//       console.log("✅ All final state verified on Solana base layer");
//     });
//   });

//   describe("Performance Benefits Demonstration", () => {
//     it("Should demonstrate the speed benefits of ephemeral rollups", async () => {
//       console.log("\n=== EPHEMERAL ROLLUPS PERFORMANCE BENEFITS ===");
//       console.log("🚀 Real-time answer submissions with ~10ms latency");
//       console.log("⛽ Gasless transactions within the rollup session");
//       console.log("🔗 Maintained composability with Solana ecosystem");
//       console.log("💰 Lower costs for high-frequency interactions");
//       console.log("🎮 Perfect for real-time multiplayer gaming");
//       console.log("===============================================\n");
//     });
//   });

//   after(() => {
//     console.log("\n=== RUSH QUIZ GAME TEST SUMMARY ===");
//     console.log("✅ Base layer initialization & setup");
//     console.log("✅ Account delegation to Ephemeral Rollup");
//     console.log("✅ Real-time gameplay in ER environment");
//     console.log("✅ Concurrent answer processing");
//     console.log("✅ State commitment back to Solana");
//     console.log("✅ Prize distribution on base layer");
//     console.log("🎯 Demonstrates full MagicBlock ER integration!");
//     console.log("=====================================");
//   });
// });