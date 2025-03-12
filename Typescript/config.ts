//config.ts
// This file contains the configuration for the scripts.

import { publicKey } from '@metaplex-foundation/umi'

export const MERKLE_MAX_DEPTH       = 14;
export const MERKLE_MAX_BUFFER_SIZE = 64;

export const METADATA_COLLECTION_URL = "https://gist.githubusercontent.com/Ayushjhax/5eb6c0cb31e506d68ff0418dc704669a/raw/72b62a4f6fdb65b56f1ec47c4edd99a16a7f4979/cnft_metadata.json";
export const METADATA_ITEM_URL       = "https://gist.githubusercontent.com/Ayushjhax/fd65289fa8ab637d4fa1b5f9226334c7/raw/c3d1c438b90b78a4447b545c4cc4cbd55ad3637d/cnft_item_metadata.json";
export const IMAGE_URL               = "https://pbs.twimg.com/profile_images/1877817218244775936/zYaaUHgY_400x400.jpg";

export const COLLECTION_NAME        = '100xDevs Collection'
export const COLLECTION_SYMBOL      = '100xDevs'
export const COLLECTION_DESCRIPTION = '100xDevs Bounty Attempted by Ayush'
export const FEE_PERCENT            = 0
export const EXTERNAL_URL           = 'https://github.com/Ayushjhax'
export const CREATORS               = [
  {
    address: publicKey('13mtmubKbZ3GNwnfGAhbos63bV3pZuxpEYDNEugPouCD'),
    verified: false,
    share: 100,
  },
]

export const NFT_ITEM_NAME      = 'Ayush Limited Edition'
export const NFT_ITEM_IMAGE_URL = IMAGE_URL;


