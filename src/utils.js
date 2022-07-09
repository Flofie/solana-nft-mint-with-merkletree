import * as anchor from '@project-serum/anchor';
import { Program, utils, web3 } from '@project-serum/anchor';
import { PublicKey } from '@solana/web3.js';
import { sendTransactions } from './connection.tsx';
import idl from './idl.json';
const {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
  MINT_SIZE,
} = require('@solana/spl-token');
const { SystemProgram } = web3;

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s'
);

const SEED = 'wallet_mint';
const NFT_CREATOR_SEED = 'NFT_CREATOR_SEED';
const OWNER = new PublicKey('2JVdWB97roMRjyTYX2qP6RxesdENVyvQNKZeERWUDnkA');
const TITLE = 'Degen Sweeper';
const SYMBOL = 'DS';
const BASE_URI =
  'https://gateway.pinata.cloud/ipfs/QmRtfaxZjX7QmwWmoih6fPQTwZpDacwvWKsMpuvwjHBg3T/';

export const _getState = async (provider, wallet) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey] = await web3.PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode(SEED))],
    program.programId
  );
  const testdata = await program.account.mintingAccount.fetch(
    walletMintingPubkey
  );

  console.log('UD:', testdata);
  let curStage = testdata.curStage;
  const ogPrice = testdata.ogPrice.div(new anchor.BN(1e6)).toNumber();
  const wlPrice = testdata.wlPrice.div(new anchor.BN(1e6)).toNumber();
  const publicPrice = testdata.publicPrice.div(new anchor.BN(1e6)).toNumber();
  const ogAmout = testdata.ogMax.toNumber();
  const wlAmout = testdata.wlMax.toNumber();
  const publicAmount = testdata.publicMax.toNumber();
  const baseUri = testdata.baseUri;
  const og_list_url = testdata.ogListUrl;
  const og_root_url = testdata.ogRootUrl;

  let isShow = true;
  // for (const og of ogList) {
  //   if (og == wallet.publicKey && curStage == 1) {
  //     isShow = true;
  //     break;
  //   }
  // }
  // for (const og of wlList) {
  //   if (og == wallet.publicKey && curStage == 2) {
  //     isShow = true;
  //     break;
  //   }
  // }
  // for (const og of blList) {
  //   if (og == wallet.publicKey) {
  //     isShow = false;
  //     curStage = 4;
  //     break;
  //   }
  // }
  // if (curStage == 3 && isShow == false) isShow = true;
  let price = publicPrice;
  if (curStage === 1) price = ogPrice;
  if (curStage === 2) price = wlPrice;
  return {
    show: isShow,
    stage: curStage,
    price: price,
    ogPrice,
    wlPrice,
    publicPrice,
    ogAmout,
    wlAmout,
    publicAmount,
    baseUri,
    ogListUrl: og_list_url,
    ogRootUrl: og_root_url,
  };
};

export const _updateOgList = async (
  provider,
  wallet,
  og_list_url,
  og_root_url,
  og_root_hash
) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.updateOgRoot(
    walletMintingBump,
    og_list_url,
    og_root_url,
    og_root_hash,
    {
      accounts: {
        mintingAccount: walletMintingPubkey,
        admin: wallet.publicKey,
      },
    }
  );
};

export const isOgList = async (provider, wallet, proof) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.isOgList(walletMintingBump, proof, {
    accounts: {
      mintingAccount: walletMintingPubkey,
      admin: wallet.publicKey,
    },
  });
};

export const _updateWlList = async (
  provider,
  wallet,
  wl_list_url,
  wl_root_url,
  wl_root_hash
) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.updateWlRoot(
    walletMintingBump,
    wl_list_url,
    wl_root_url,
    wl_root_hash,
    {
      accounts: {
        mintingAccount: walletMintingPubkey,
        admin: wallet.publicKey,
      },
    }
  );
};

export const isWlList = async (provider, wallet, proof) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.isWlList(walletMintingBump, proof, {
    accounts: {
      mintingAccount: walletMintingPubkey,
      admin: wallet.publicKey,
    },
  });
};

export const updatePrice = async (
  provider,
  wallet,
  ogPrice,
  wlPrice,
  blPrice
) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.updatePrice(
    walletMintingBump,
    new anchor.BN(ogPrice).mul(new anchor.BN(1e6)),
    new anchor.BN(wlPrice).mul(new anchor.BN(1e6)),
    new anchor.BN(blPrice).mul(new anchor.BN(1e6)),
    {
      accounts: {
        mintingAccount: walletMintingPubkey,
        admin: wallet.publicKey,
      },
    }
  );
};

export const updateAmount = async (
  provider,
  wallet,
  ogPrice,
  wlPrice,
  blPrice
) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.updateAmount(
    walletMintingBump,
    new anchor.BN(ogPrice),
    new anchor.BN(wlPrice),
    new anchor.BN(blPrice),
    {
      accounts: {
        mintingAccount: walletMintingPubkey,
        admin: wallet.publicKey,
      },
    }
  );
};

export const setStage = async (provider, wallet, newStage) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.setStage(walletMintingBump, newStage, {
    accounts: {
      mintingAccount: walletMintingPubkey,
      admin: wallet.publicKey,
    },
  });
};

export const setUri = async (provider, wallet, newUri) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey, walletMintingBump] =
    await web3.PublicKey.findProgramAddress(
      [Buffer.from(utils.bytes.utf8.encode(SEED))],
      program.programId
    );
  await program.rpc.setUri(walletMintingBump, newUri, {
    accounts: {
      mintingAccount: walletMintingPubkey,
      admin: wallet.publicKey,
    },
  });
};

export const initialize = async (provider, wallet) => {
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [stakingPubkey] = await web3.PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode(SEED))],
    program.programId
  );
  await program.rpc.initialize(
    new anchor.BN(9999),
    new anchor.BN(3),
    new anchor.BN(4),
    new anchor.BN(1),
    new anchor.BN(5e8),
    new anchor.BN(4e8),
    new anchor.BN(1e8),
    TITLE,
    SYMBOL,
    BASE_URI,
    {
      accounts: {
        mintingAccount: stakingPubkey,
        initializer: wallet.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      },
    }
  );
};

export const multiMint = async (provider, wallet, count, proof, wlmint) => {
  const { SystemProgram } = web3;
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintPubkey] = await web3.PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode(SEED))],
    program.programId
  );
  const [makerPubkey] = await web3.PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode(NFT_CREATOR_SEED))],
    program.programId
  );
  const [userMintingPubkey] = await web3.PublicKey.findProgramAddress(
    [wallet.publicKey.toBuffer()],
    program.programId
  );

  let lamports =
    await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );

  // const collection = new PublicKey(
  //   'DX5ezYq7QbPSW5UYHnhS9obmquZvrwR9oSYbBKioruqR'
  // );
  const getMetadata = async (mint) => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };
  const getMasterEdition = async (mint) => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
          Buffer.from('edition'),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };

  const signersMatrix = [];
  const instructionsMatrix = [];
  for (let index = 0; index < count; index++) {
    const mintKey = anchor.web3.Keypair.generate();
    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      provider.wallet.publicKey
    );
    const signers = [mintKey];
    const instructions = [
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        wallet.publicKey,
        wallet.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        NftTokenAccount,
        wallet.publicKey,
        mintKey.publicKey
      ),
    ];
    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);
    const txinstruction = wlmint
      ? program.instruction.mintNftWl(proof, {
          accounts: {
            mintAuthority: wallet.publicKey,
            mint: mintKey.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            metadata: metadataAddress,
            tokenAccount: NftTokenAccount,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            payer: wallet.publicKey,
            owner: OWNER,
            mintingAccount: walletMintPubkey,
            userMintingCounterAccount: userMintingPubkey,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            masterEdition: masterEdition,
            maker: makerPubkey,
          },
        })
      : program.instruction.mintNft({
          accounts: {
            mintAuthority: wallet.publicKey,
            mint: mintKey.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            metadata: metadataAddress,
            tokenAccount: NftTokenAccount,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
            payer: wallet.publicKey,
            owner: OWNER,
            mintingAccount: walletMintPubkey,
            userMintingCounterAccount: userMintingPubkey,
            systemProgram: SystemProgram.programId,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            masterEdition: masterEdition,
            maker: makerPubkey,
          },
        });
    instructions.push(txinstruction);
    signersMatrix.push(signers);
    instructionsMatrix.push(instructions);
  }
  try {
    return (
      await sendTransactions(
        provider.connection,
        provider.wallet,
        instructionsMatrix,
        signersMatrix
      )
    ).txs.map((t) => t.txid);
  } catch (e) {
    console.log(e);
    return false;
  }
};

export const mintCollection = async (provider, wallet) => {
  const { SystemProgram } = web3;
  const programID = new PublicKey(idl.metadata.address);
  const program = new Program(idl, programID, provider);
  const [walletMintingPubkey] = await web3.PublicKey.findProgramAddress(
    [Buffer.from(utils.bytes.utf8.encode(SEED))],
    program.programId
  );
  let lamports =
    await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );
  const getMetadata = async (mint) => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };
  const getMasterEdition = async (mint) => {
    return (
      await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from('metadata'),
          TOKEN_METADATA_PROGRAM_ID.toBuffer(),
          mint.toBuffer(),
          Buffer.from('edition'),
        ],
        TOKEN_METADATA_PROGRAM_ID
      )
    )[0];
  };

  const signersMatrix = [];
  const instructionsMatrix = [];
  const mintKey = anchor.web3.Keypair.generate();
  console.log('=== collection mint key ===', mintKey.publicKey.toBase58());
  const NftTokenAccount = await getAssociatedTokenAddress(
    mintKey.publicKey,
    provider.wallet.publicKey
  );
  const signers = [mintKey];
  const cleanupInstructions = [];
  const instructions = [
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: wallet.publicKey,
      newAccountPubkey: mintKey.publicKey,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
      lamports,
    }),
    createInitializeMintInstruction(
      mintKey.publicKey,
      0,
      wallet.publicKey,
      wallet.publicKey
    ),
    createAssociatedTokenAccountInstruction(
      wallet.publicKey,
      NftTokenAccount,
      wallet.publicKey,
      mintKey.publicKey
    ),
  ];
  const metadataAddress = await getMetadata(mintKey.publicKey);
  const masterEdition = await getMasterEdition(mintKey.publicKey);
  instructions.push(
    program.instruction.mintCollectionNft({
      accounts: {
        admin: wallet.publicKey,
        mint: mintKey.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadata: metadataAddress,
        tokenAccount: NftTokenAccount,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        payer: wallet.publicKey,
        owner: OWNER,
        mintingAccount: walletMintingPubkey,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        masterEdition: masterEdition,
      },
    })
  );
  signersMatrix.push(signers);
  instructionsMatrix.push(instructions);
  if (cleanupInstructions.length > 0) {
    instructionsMatrix.push(cleanupInstructions);
    signersMatrix.push([]);
  }
  try {
    return (
      await sendTransactions(
        provider.connection,
        provider.wallet,
        instructionsMatrix,
        signersMatrix
      )
    ).txs.map((t) => t.txid);
  } catch (e) {
    console.log(e);
    return false;
  }
};
