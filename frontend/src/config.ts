import { Networks } from "@stellar/stellar-sdk";

export const CONFIG = {
  mainnet: {
    testMode: false,
    rpcUrl: "https://horizon.stellar.org",
    passphrase: Networks.PUBLIC,
    contractAddress: "CB27BB4AYGLJ4AVQSHIEXW6ABTSJON27I3VLQNGJM7T3AZPBRPLKJIK5",
    nativeToken: {
      code: "XLM",
      address: "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA", // XLM Address
      issuer: "", // XLM Address
    },
    outputToken: {
      code: "USDC",
      address: "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75", // USDC Address
      issuer: "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN", // USDC Address
    },
  },
  testnet: {
    testMode: true,
    rpcUrl: "https://horizon-testnet.stellar.org",
    passphrase: Networks.TESTNET,
    contractAddress: "CDRYDKVD7V6OEKUUIRUWUV354XEYI4CNS3P76RE2VPIEVG5D3AFVXMHP",
    nativeToken: {
      code: "XLM",
      address: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", // XLM Address
      issuer: "", // XLM Address
    },
    outputToken: {
      code: "USDC",
      address: "CAS7WHY4XXBESA4QIYQVL52AZFTJPUEQ5BXPES6OF4PI5JA3L7RM5Q2B", // Soroswap USDC Address https://github.com/soroswap/core/blob/main/public/tokens.json
      issuer: "GCPJFNZAARY3Z2AM7RVXDZDLPOEBT4QHTQXFOFKMZHLV7PPDKE2M67Q6", // Soroswap USDC Address
    },
  },
};
