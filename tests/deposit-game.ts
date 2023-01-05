const assert = require("assert");
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
const { TOKEN_PROGRAM_ID } = require("@solana/spl-token");
import { DepositGame } from '../target/types/deposit_game';

import poolSecret from '../pool.json';

async function sendLamports(
    provider,
    destination,
    amount
) {
    const tx = new anchor.web3.Transaction();
    tx.add(
        anchor.web3.SystemProgram.transfer(
            { 
                fromPubkey: provider.wallet.publicKey, 
                lamports: amount, 
                toPubkey: destination
            }
        )
    );
    await provider.send(tx);
}

describe('deposit-game', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.DepositGame as Program<DepositGame>;
  const provider = anchor.getProvider();

  let poolKeypair = anchor.web3.Keypair.generate();
  let poolPubkey = poolKeypair.publicKey;
  let user1 = anchor.web3.Keypair.generate();
  let user2 = anchor.web3.Keypair.generate();
  let user3 = anchor.web3.Keypair.generate();
  let user4 = anchor.web3.Keypair.generate();
  let user5 = anchor.web3.Keypair.generate();
  let user6 = anchor.web3.Keypair.generate();
  let user7 = anchor.web3.Keypair.generate();
  let user8 = anchor.web3.Keypair.generate();
  let user9 = anchor.web3.Keypair.generate();
  let user10 = anchor.web3.Keypair.generate();
  let gameId1;

  it('Is initialized!', async () => {
    const [
        poolSigner,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(),
        ],
        program.programId
    );

    await sendLamports(provider, user1.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user2.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user3.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user4.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user5.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user6.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user7.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user8.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user9.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await sendLamports(provider, user10.publicKey, anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.rpc.initialize(nonce, {
      accounts: {
        authority: provider.wallet.publicKey,
        pool: poolKeypair.publicKey,
        poolSigner: poolSigner,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [poolKeypair, ],
      instructions: [
          await program.account.pool.createInstruction(poolKeypair, ),
      ],
    });
    console.log("Your transaction signature", tx);
  });

  it('create game 1', async () => {
    const envProvider = anchor.Provider.env();
    const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user1), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    let poolObject = await userProgram.account.pool.fetch(poolPubkey);
    gameId1 = (poolObject.gameCount.toNumber() + 1).toString();

    const [
        game,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
        vault,
        nonceVault,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("vault"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    let gameObject;
    try {
        gameObject = await userProgram.account.game.fetch(game);
    } catch (e) {
        console.log(e.message)
    }
    let instructions = [];
    if(!gameObject) {
        instructions.push(
            await userProgram.instruction.createGame(nonce, nonceVault, gameId1, {
                accounts: {
                    pool: poolPubkey,
                    game: game,
                    vault: vault,
                    signer: user1.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }
            })
        )
    }

    const transaction = new anchor.web3.Transaction().add(...instructions);
      var signature = await anchor.web3.sendAndConfirmTransaction(
        p.connection,
        transaction,
        [user1]
      );
  })

  it('deposit1', async () => {
    const envProvider = anchor.Provider.env();
    const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user1), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    const [
        _poolSigner,
        _nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(),
        ],
        userProgram.programId
    );

    const [
        game,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
        vault,
        nonceVault,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("vault"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const gameObject = await userProgram.account.game.fetch(game);
    const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
    const tx = await userProgram.rpc.deposit(new anchor.BN(amount), {
      accounts: {
        pool: poolPubkey,
        game: game,
        vault: vault,
        depositor: user1.publicKey,
        poolSigner: _poolSigner,
        account1: gameObject.account1,
        account2: gameObject.account2,
        account3: gameObject.account3,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    let contractLamports = (await provider.connection.getBalance(vault));
    assert.equal(contractLamports, amount);
  })

  it('deposit2', async () => {
    const envProvider = anchor.Provider.env();
    const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user2), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    const [
        _poolSigner,
        _nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(),
        ],
        userProgram.programId
    );

    const [
        game,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
        vault,
        nonceVault,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("vault"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const gameObject = await userProgram.account.game.fetch(game);
    const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
    const tx = await userProgram.rpc.deposit(new anchor.BN(amount), {
      accounts: {
        pool: poolPubkey,
        game: game,
        vault: vault,
        depositor: user2.publicKey,
        poolSigner: _poolSigner,
        account1: gameObject.account1,
        account2: gameObject.account2,
        account3: gameObject.account3,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    let contractLamports = (await provider.connection.getBalance(vault));
    assert.equal(contractLamports, amount * 2);
  })

  it('deposit3', async () => {
    const envProvider = anchor.Provider.env();
    const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user3), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    const [
        _poolSigner,
        _nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(),
        ],
        userProgram.programId
    );

    const [
        game,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const [
        vault,
        nonceVault,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("vault"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const gameObject = await userProgram.account.game.fetch(game);
    const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
    const tx = await userProgram.rpc.deposit(new anchor.BN(amount), {
      accounts: {
        pool: poolPubkey,
        game: game,
        vault: vault,
        depositor: user3.publicKey,
        poolSigner: _poolSigner,
        account1: gameObject.account1,
        account2: gameObject.account2,
        account3: gameObject.account3,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    let contractLamports = (await provider.connection.getBalance(vault));
    assert.equal(contractLamports, 0);

    let user1Lamports = (await provider.connection.getBalance(user1.publicKey));
    let user2Lamports = (await provider.connection.getBalance(user2.publicKey));
    let user3Lamports = (await provider.connection.getBalance(user3.publicKey));
    console.log("user1 Lamports: ", user1Lamports);
    console.log("user2 Lamports: ", user2Lamports);
    console.log("user3 Lamports: ", user3Lamports);
  })

  it('check last hash', async () => {
    const envProvider = anchor.Provider.env();
    const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user3), envProvider.opts);

    const userProgram = new anchor.Program(program.idl, program.programId, p);

    const [
        game,
        nonce,
    ] = await anchor.web3.PublicKey.findProgramAddress(
        [
          poolPubkey.toBuffer(), Buffer.from("game"), Buffer.from(gameId1)
        ],
        userProgram.programId
    );

    const gameObject = await userProgram.account.game.fetch(game);
    let hexString = "";
    for(let i = 0;i < gameObject.lastHash.length;i ++) {
      hexString += Buffer.from([gameObject.lastHash[i]]).toString('hex')
    }
    console.log(hexString)
    console.log(Buffer.from(gameObject.lastHash).toString('hex'));
  })

  // it('deposit5-1', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user1), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game5,
  //       _nonce5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game5")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault5,
  //       _nonceVault5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault5")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault5));
  //   console.log("Game5 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game5.fetch(game5);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit5(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game5,
  //       vault: vault5,
  //       depositor: user1.publicKey,
  //       gameSigner: game5,
  //       vaultSigner: vault5,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault5));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit5-2', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user2), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game5,
  //       _nonce5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game5")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault5,
  //       _nonceVault5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault5")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault5));
  //   console.log("Game5 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game5.fetch(game5);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit5(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game5,
  //       vault: vault5,
  //       vaultSigner: vault5,
  //       depositor: user2.publicKey,
  //       gameSigner: game5,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault5));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit5-3', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user3), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game5,
  //       _nonce5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game5")
  //       ],
  //       program.programId
  //   );


  //   const [
  //       vault5,
  //       _nonceVault5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault5")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault5));
  //   console.log("Game5 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game5.fetch(game5);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit5(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game5,
  //       vault: vault5,
  //       vaultSigner: vault5,
  //       depositor: user3.publicKey,
  //       gameSigner: game5,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault5));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit5-4', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user4), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game5,
  //       _nonce5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game5")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault5,
  //       _nonceVault5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault5")
  //       ],
  //       program.programId
  //   );


  //   let prevContractLamports = (await provider.connection.getBalance(vault5));
  //   console.log("Game5 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game5.fetch(game5);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit5(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game5,
  //       vault: vault5,
  //       vaultSigner: vault5,
  //       depositor: user4.publicKey,
  //       gameSigner: game5,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault5));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit5-5', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user5), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game5,
  //       _nonce5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game5")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault5,
  //       _nonceVault5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault5")
  //       ],
  //       program.programId
  //   );


  //   let prevContractLamports = (await provider.connection.getBalance(vault5));
  //   console.log("Game5 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game5.fetch(game5);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit5(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game5,
  //       vault: vault5,
  //       vaultSigner: vault5,
  //       depositor: user5.publicKey,
  //       gameSigner: game5,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault5));
  //   assert.equal(contractLamports, 0);
  //   let user1Lamports = (await provider.connection.getBalance(user1.publicKey));
  //   let user2Lamports = (await provider.connection.getBalance(user2.publicKey));
  //   let user3Lamports = (await provider.connection.getBalance(user3.publicKey));
  //   let user4Lamports = (await provider.connection.getBalance(user4.publicKey));
  //   let user5Lamports = (await provider.connection.getBalance(user5.publicKey));
  //   console.log("user1 Lamports: ", user1Lamports);
  //   console.log("user2 Lamports: ", user2Lamports);
  //   console.log("user3 Lamports: ", user3Lamports);
  //   console.log("user4 Lamports: ", user4Lamports);
  //   console.log("user5 Lamports: ", user5Lamports);
  // })

  // it('check game5 last hash', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user3), envProvider.opts);

  //   const [
  //       game5,
  //       _nonce5,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game5")
  //       ],
  //       program.programId
  //   );

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const game = await userProgram.account.game5.fetch(game5);

  //   console.log("Game5 Last Hash: ", Buffer.from(game.lastHash).toString('hex'));
  // })

  // it('deposit10-1', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user1), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user1.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-2', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user2), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user2.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-3', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user3), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user3.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-4', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user4), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user4.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-5', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user5), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user5.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-6', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user6), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user6.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-7', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user7), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user7.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-8', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user8), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user8.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-9', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user9), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user9.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, prevContractLamports + amount);
  // })

  // it('deposit10-10', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user10), envProvider.opts);

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const [
  //       vault10,
  //       _nonceVault10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("vault10")
  //       ],
  //       program.programId
  //   );

  //   let prevContractLamports = (await provider.connection.getBalance(vault10));
  //   console.log("Game10 Balance: ", prevContractLamports)

  //   const game = await userProgram.account.game10.fetch(game10);
  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await userProgram.rpc.deposit10(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolPubkey,
  //       game: game10,
  //       vault: vault10,
  //       vaultSigner: vault10,
  //       depositor: user10.publicKey,
  //       gameSigner: game10,
  //       account1: game.account1,
  //       account2: game.account2,
  //       account3: game.account3,
  //       account4: game.account4,
  //       account5: game.account5,
  //       account6: game.account6,
  //       account7: game.account7,
  //       account8: game.account8,
  //       account9: game.account9,
  //       account10: game.account10,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(vault10));
  //   assert.equal(contractLamports, 0);
  //   let user1Lamports = (await provider.connection.getBalance(user1.publicKey));
  //   let user2Lamports = (await provider.connection.getBalance(user2.publicKey));
  //   let user3Lamports = (await provider.connection.getBalance(user3.publicKey));
  //   let user4Lamports = (await provider.connection.getBalance(user4.publicKey));
  //   let user5Lamports = (await provider.connection.getBalance(user5.publicKey));
  //   let user6Lamports = (await provider.connection.getBalance(user6.publicKey));
  //   let user7Lamports = (await provider.connection.getBalance(user7.publicKey));
  //   let user8Lamports = (await provider.connection.getBalance(user8.publicKey));
  //   let user9Lamports = (await provider.connection.getBalance(user9.publicKey));
  //   let user10Lamports = (await provider.connection.getBalance(user10.publicKey));
  //   console.log("user1 Lamports: ", user1Lamports);
  //   console.log("user2 Lamports: ", user2Lamports);
  //   console.log("user3 Lamports: ", user3Lamports);
  //   console.log("user4 Lamports: ", user4Lamports);
  //   console.log("user5 Lamports: ", user5Lamports);
  //   console.log("user6 Lamports: ", user6Lamports);
  //   console.log("user7 Lamports: ", user7Lamports);
  //   console.log("user8 Lamports: ", user8Lamports);
  //   console.log("user9 Lamports: ", user9Lamports);
  //   console.log("user10 Lamports: ", user10Lamports);
  // })

  // it('check game10 last hash', async () => {
  //   const envProvider = anchor.Provider.env();
  //   const p = new anchor.Provider(envProvider.connection, new anchor.Wallet(user3), envProvider.opts);

  //   const [
  //       game10,
  //       _nonce10,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         poolPubkey.toBuffer(), Buffer.from("game10")
  //       ],
  //       program.programId
  //   );

  //   const userProgram = new anchor.Program(program.idl, program.programId, p);

  //   const game = await userProgram.account.game10.fetch(game10);

  //   console.log("Game10 Last Hash: ", Buffer.from(game.lastHash).toString('hex'));
  // })

});
