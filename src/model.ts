import { PublicKey } from "@metaplex-foundation/js";


export class DistData {
    token_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
    distribution_open: number = 0;
    tokens_left: number = 0;
    bump: number = 0;
  
    constructor(fields: {
      token_mint?: number[];
      distribution_open: number;
      tokens_left: number;
      bump: number;

    } | undefined = undefined) {
      if (fields) {
        if (fields.token_mint) this.token_mint = fields.token_mint;
        this.distribution_open = fields.distribution_open;
        this.tokens_left = fields.tokens_left;
        this.bump = fields.bump;
      }
    }
}
export class NFTTerms {
    owner: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
    nft_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
    tokenization_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
    for_sale: number = 0;
    tokenized_for_sale: number = 0;
    buy_out_allowed: number = 0;
    owned_by_pda: number = 0;
    price: bigint = BigInt(0);
    buy_out_price: bigint = BigInt(0);
    lamports_per_token_buyout: bigint = BigInt(0);
    number_of_tokens: bigint = BigInt(0);
    lamports_per_token: bigint = BigInt(0);
    tokens_sold: bigint = BigInt(0);
    bump: number = 0;
    vote_open:number = 0;

    constructor(fields?: {
        owner?: number[];
        nft_mint?: number[];
        tokenization_mint?: number[];
        for_sale?: number;
        tokenized_for_sale?: number;
        buy_out_allowed?: number;
        owned_by_pda?: number;
        price?: bigint;
        buy_out_price?: bigint;
        lamports_per_token_buyout?: bigint;
        number_of_tokens?: bigint;
        lamports_per_token?: bigint;
        tokens_sold?: bigint;
        bump?: number;
        vote_open:number;
  }) {
        if (fields) {
            if (fields.owner) this.owner = fields.owner;
            if (fields.nft_mint) this.nft_mint = fields.nft_mint;
            if (fields.tokenization_mint) this.tokenization_mint = fields.tokenization_mint;
            if (fields.for_sale !== undefined) this.for_sale = fields.for_sale;
            if (fields.tokenized_for_sale !== undefined) this.tokenized_for_sale = fields.tokenized_for_sale;
            if (fields.buy_out_allowed !== undefined) this.buy_out_allowed = fields.buy_out_allowed;
            if (fields.owned_by_pda !== undefined) this.owned_by_pda = fields.owned_by_pda;
            if (fields.price !== undefined) this.price = fields.price;
            if (fields.buy_out_price !== undefined) this.buy_out_price = fields.buy_out_price;
            if (fields.lamports_per_token_buyout !== undefined) this.lamports_per_token_buyout = fields.lamports_per_token_buyout;
            if (fields.number_of_tokens !== undefined) this.number_of_tokens = fields.number_of_tokens;
            if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
            if (fields.tokens_sold !== undefined) this.tokens_sold = fields.tokens_sold;
            if (fields.bump !== undefined) this.bump = fields.bump;
            if (fields.vote_open !== undefined) this.vote_open = fields.vote_open;
        }
    }
}
export class FunderAccount {
  funder: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  nft_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  tokens_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  fund_invested: bigint = BigInt(0);
  lamports_per_token: bigint = BigInt(0);

  constructor(fields?: {
      funder?: number[];
      nft_mint?: number[];
      tokens_mint?: number[];
      fund_invested?: bigint;
      lamports_per_token: bigint;
}) {
      if (fields) {
          if (fields.funder) this.funder = fields.funder;
          if (fields.nft_mint) this.nft_mint = fields.nft_mint;
          if (fields.tokens_mint) this.tokens_mint = fields.tokens_mint;
          if (fields.fund_invested !== undefined) this.fund_invested = fields.fund_invested;
          if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
      }
  }
}
export class InitAccount {
  bump: number = 0;
  size: bigint = BigInt(0);
  lamports: bigint = BigInt(0);

  constructor(fields?: {
      size?: bigint;
      lamports?: bigint;
      bump?: number;

  }) {
      if (fields) {
          if (fields.size !== undefined) this.size = fields.size;
          if (fields.lamports !== undefined) this.lamports = fields.lamports;
          if (fields.bump !== undefined) this.bump = fields.bump;
      }
  }
}
export class FundRaising {
  fund_raising: number = 0;
  nft_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  tokens_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  funds_collected: bigint = BigInt(0);
  number_of_tokens: bigint = BigInt(0);
  lamports_per_token: bigint = BigInt(0);
  bump: number = 0;

  constructor(fields?: {
      fund_raising?: number;
      nft_mint?: number[];
      tokens_mint?: number[];
      funds_collected?: bigint;
      number_of_tokens?: bigint;
      lamports_per_token?: bigint;
      bump?: number;
  }) {
      if (fields) {
          if (fields.fund_raising !== undefined) this.fund_raising = fields.fund_raising;
          if (fields.nft_mint) this.nft_mint = fields.nft_mint;
          if (fields.tokens_mint) this.tokens_mint = fields.tokens_mint;
          if (fields.funds_collected !== undefined) this.funds_collected = fields.funds_collected;
          if (fields.number_of_tokens !== undefined) this.number_of_tokens = fields.number_of_tokens;
          if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
          if (fields.bump !== undefined) this.bump = fields.bump;
      }
  }
}
export class InitPDA {
  init_pda: number = 0;

  constructor(fields?: {
      init_pda?: number;
  }) {
      if (fields) {
          if (fields.init_pda !== undefined) this.init_pda = fields.init_pda;
      }
  }
}
export class Lamports {
  lamports: bigint = BigInt(0);

  constructor(fields?: {
      lamports?: bigint;
  }) {
      if (fields) {
          if (fields.lamports !== undefined) this.lamports = fields.lamports;
      }
  }
}
export class StartVoting {
  offer: bigint = BigInt(0);
  proposer_pda: number = 0;
  vote_account_pda: number = 0;

  constructor(fields?: {
      offer?: bigint;
      proposer_pda?: number;
      vote_account_pda?: number;
  }) {
      if (fields) {
          if (fields.offer !== undefined) this.offer = fields.offer;
          if (fields.proposer_pda !== undefined) this.proposer_pda = fields.proposer_pda;
          if (fields.vote_account_pda !== undefined) this.vote_account_pda = fields.vote_account_pda;
      }
  }
}
export class Terms {
  is_init: number = 0;
  funder_account: bigint = BigInt(0);
  funder_account_size: bigint = BigInt(0);
  fundrasing_account: bigint = BigInt(0);
  fundrasing_account_size: bigint = BigInt(0);
  nft_pda_account: bigint = BigInt(0);
  nft_pda_account_size: bigint = BigInt(0);
  vote_account: bigint = BigInt(0);
  vote_account_size: bigint = BigInt(0);
  proposal_account: bigint = BigInt(0);
  proposal_account_size: bigint = BigInt(0);
  mint: bigint = BigInt(0);
  mint_size: bigint = BigInt(0);
  token_distribution_account: bigint = BigInt(0);
  token_distribution_account_size: bigint = BigInt(0);
  usertokenmint_account: bigint = BigInt(0);
  usertokenmint_account_size: bigint = BigInt(0);
  token_to_sol_account: bigint = BigInt(0);
  token_to_sol_account_size: bigint = BigInt(0);
  investor_account: bigint = BigInt(0);
  investor_account_size: bigint = BigInt(0);
  lamports_per_token_fundraising: bigint = BigInt(0);
  minimum_lamports_per_token: bigint = BigInt(0);
  buy_sell_order_account:bigint = BigInt(0);
  buy_sell_order_account_size:bigint = BigInt(0);
  small_account:bigint = BigInt(0);

  constructor(fields?: {
      is_init?: number;
      funder_account?: bigint;
      funder_account_size?: bigint;
      fundrasing_account?: bigint;
      fundrasing_account_size?: bigint;
      nft_pda_account?: bigint;
      nft_pda_account_size?: bigint;
      vote_account?: bigint;
      vote_account_size?: bigint;
      proposal_account?: bigint;
      proposal_account_size?: bigint;
      tokenization_account?: bigint;
      tokenization_account_size?: bigint;
      token_distribution_account?: bigint;
      token_distribution_account_size?: bigint;
      usertokenmint_account?: bigint;
      usertokenmint_account_size?: bigint;
      token_to_sol_account?: bigint;
      token_to_sol_account_size?: bigint;
      investor_account?: bigint ;
      investor_account_size?: bigint ;
      lamports_per_token_fundraising?: bigint;
      minimum_lamports_per_token?: bigint;
      buy_sell_order_account:bigint;
      buy_sell_order_account_size:bigint;
      small_account:bigint;

  }) {
      if (fields) {
          if (fields.is_init !== undefined) this.is_init = fields.is_init;
          if (fields.funder_account !== undefined) this.funder_account = fields.funder_account;
          if (fields.funder_account_size !== undefined) this.funder_account_size = fields.funder_account_size;
          if (fields.fundrasing_account !== undefined) this.fundrasing_account = fields.fundrasing_account;
          if (fields.fundrasing_account_size !== undefined) this.fundrasing_account_size = fields.fundrasing_account_size;
          if (fields.nft_pda_account !== undefined) this.nft_pda_account = fields.nft_pda_account;
          if (fields.nft_pda_account_size !== undefined) this.nft_pda_account_size = fields.nft_pda_account_size;
          if (fields.vote_account !== undefined) this.vote_account = fields.vote_account;
          if (fields.vote_account_size !== undefined) this.vote_account_size = fields.vote_account_size;
          if (fields.proposal_account !== undefined) this.proposal_account = fields.proposal_account;
          if (fields.proposal_account_size !== undefined) this.proposal_account_size = fields.proposal_account_size;
          if (fields.tokenization_account !== undefined) this.mint = fields.tokenization_account;
          if (fields.tokenization_account_size !== undefined) this.mint_size = fields.tokenization_account_size;
          if (fields.token_distribution_account !== undefined) this.token_distribution_account = fields.token_distribution_account;
          if (fields.token_distribution_account_size !== undefined) this.token_distribution_account_size = fields.token_distribution_account_size;
          if (fields.usertokenmint_account !== undefined) this.usertokenmint_account = fields.usertokenmint_account;
          if (fields.usertokenmint_account_size !== undefined) this.usertokenmint_account_size = fields.usertokenmint_account_size;
          if (fields.token_to_sol_account !== undefined) this.token_to_sol_account = fields.  token_to_sol_account;
          if (fields.token_to_sol_account_size !== undefined) this.token_to_sol_account_size = fields.token_to_sol_account_size;
          if (fields.investor_account !== undefined) this.investor_account = fields.investor_account;
          if (fields.token_to_sol_account_size !== undefined) this.token_to_sol_account_size = fields.token_to_sol_account_size;
          if (fields.investor_account_size !== undefined) this.investor_account_size = fields. investor_account_size;
          if (fields.minimum_lamports_per_token !== undefined) this.minimum_lamports_per_token = fields. minimum_lamports_per_token;
          if (fields.buy_sell_order_account !== undefined) this.buy_sell_order_account = fields. buy_sell_order_account;
          if (fields.buy_sell_order_account_size !== undefined) this.buy_sell_order_account_size = fields. buy_sell_order_account_size;
          if (fields.small_account !== undefined) this.small_account = fields. small_account;

      }
  }
}
export class UserAddresTokenMint {
  owner: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  token_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  constructor(fields?: {
    owner?: number[];
    token_mint?: number[];
  }) {
      if (fields) {
        if (fields.owner) this.owner = fields.owner;
        if (fields.token_mint) this.token_mint = fields.token_mint;
      }
  }
}
export class NFTonSale {
    owner: PublicKey = new PublicKey("");
    nft_mint: PublicKey = new PublicKey("");
    tokenization_mint: PublicKey = new PublicKey("");
    for_sale: number = 0;
    buy_out_allowed: number = 0;
    owned_by_pda: number = 0;
    price: bigint = BigInt(0);
    bump: number = 0;
    lamports_per_token_buyout: bigint = BigInt(0);
    number_of_tokens: bigint = BigInt(0);
    lamports_per_token: bigint = BigInt(0);
    tokens_sold: bigint = BigInt(0);


    constructor(fields?: {
        owner?: PublicKey;
        nft_mint?: PublicKey;
        tokenization_mint?: PublicKey;
        for_sale?: number;
        buy_out_allowed?: number;
        price?: bigint;
        bump?: number;
        lamports_per_token_buyout: bigint;
        number_of_tokens: bigint;
        lamports_per_token: bigint;
        tokens_sold: bigint;
  }) {
        if (fields) {
            if (fields.owner) this.owner = fields.owner;
            if (fields.nft_mint) this.nft_mint = fields.nft_mint;
            if (fields.tokenization_mint) this.tokenization_mint = fields.tokenization_mint;
            if (fields.for_sale !== undefined) this.for_sale = fields.for_sale;
            if (fields.buy_out_allowed !== undefined) this.buy_out_allowed = fields.buy_out_allowed;
            if (fields.price !== undefined) this.price = fields.price;
            if (fields.bump !== undefined) this.bump = fields.bump;
            if (fields.lamports_per_token_buyout !== undefined) this.lamports_per_token_buyout = fields.lamports_per_token_buyout;
            if (fields.number_of_tokens !== undefined) this.number_of_tokens = fields.number_of_tokens;
            if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
            if (fields.tokens_sold !== undefined) this.tokens_sold = fields.tokens_sold;

        }
    }
}
export class ActiveFundRaising {
  fund_raising: number = 0;
  nft_mint: PublicKey = new PublicKey("");
  tokens_mint: PublicKey = new PublicKey("");
  funds_collected: bigint = BigInt(0);
  number_of_tokens: bigint = BigInt(0);
  lamports_per_token: bigint = BigInt(0);
  bump: number = 0;

  constructor(fields?: {
      fund_raising?: number;
      nft_mint?: PublicKey;
      tokens_mint?: PublicKey;
      funds_collected?: bigint;
      number_of_tokens?: bigint;
      lamports_per_token?: bigint;
      bump?: number;
  }) {
      if (fields) {
          if (fields.fund_raising !== undefined) this.fund_raising = fields.fund_raising;
          if (fields.nft_mint) this.nft_mint = fields.nft_mint;
          if (fields.tokens_mint) this.tokens_mint = fields.tokens_mint;
          if (fields.funds_collected !== undefined) this.funds_collected = fields.funds_collected;
          if (fields.number_of_tokens !== undefined) this.number_of_tokens = fields.number_of_tokens;
          if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
          if (fields.bump !== undefined) this.bump = fields.bump;
      }
  }
}
export class MyFundRaising {
  fund_raising_succesful: boolean = false;
  nft_mint: PublicKey = new PublicKey("");
  tokens_mint: PublicKey = new PublicKey("");
  funders_account: PublicKey = new PublicKey("");
  funds_invested: bigint = BigInt(0);
  tokens_to_receive: bigint = BigInt(0);
  funds_collected: bigint = BigInt(0);
  number_of_tokens: bigint = BigInt(0);
  lamports_per_token: bigint = BigInt(0);
  bump: number = 0;

  constructor(fields?: {
    fund_raising_succesful?: boolean;
      nft_mint?: PublicKey;
      tokens_mint?: PublicKey;
      funders_account?: PublicKey;
      funds_invested?: bigint;
      tokens_to_receive?: bigint;
      funds_collected?: bigint;
      number_of_tokens?: bigint;
      lamports_per_token?: bigint;
      bump?: number;
  }) {
      if (fields) {
          if (fields.fund_raising_succesful !== undefined) this.fund_raising_succesful = fields.fund_raising_succesful;
          if (fields.nft_mint) this.nft_mint = fields.nft_mint;
          if (fields.tokens_mint) this.tokens_mint = fields.tokens_mint;
          if (fields.funds_collected !== undefined) this.funds_collected = fields.funds_collected;
          if (fields.number_of_tokens !== undefined) this.number_of_tokens = fields.number_of_tokens;
          if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
          if (fields.bump !== undefined) this.bump = fields.bump;
      }
  }
}
export class NFT {

  nft_mint: PublicKey = new PublicKey("");
  metadata: PublicKey = new PublicKey("");
  url:String = "";

  constructor(fields?: {
      nft_mint?: PublicKey;
      metadata?: PublicKey;
      url:String;
  }) {
      if (fields) {
          if (fields.nft_mint) this.nft_mint = fields.nft_mint;
          if (fields.metadata) this.metadata = fields.metadata;
          if (fields.url !== undefined) this.url = fields.url;
      }
  }
}

export const NFTTermsSchema=new Map([
  [
    NFTTerms,
    {
      kind: "struct",
      fields: [
        ["owner",["u8",32]],
        ["nft_mint",["u8",32]],
        ["tokenization_mint",["u8",32]],
        ["for_sale","u8"], 
        ["tokenized_for_sale","u8"], 
        ["buy_out_allowed","u8"], 
        ["owned_by_pda","u8"], 
        ["price","u64"], 
        ["buy_out_price","u64"], 
        ["lamports_per_token_buyout","u64"], 
        ["number_of_tokens","u64"], 
        ["lamports_per_token","u64"], 
        ["tokens_sold","u64"], 
        ["bump","u8"], 
        ["vote_open","u8"], 
  
      ],
    },
  ],
])
export const DistDataSchema = new Map([
  [
    DistData,
    {
      kind: 'struct',
      fields: [
        ['token_mint', ['u8', 32]],
        ['distribution_open', 'u8'],
        ['tokens_left', 'u64'],
        ['bump', 'u8'],
      ],
    },
  ],
]);
export const FunderAccountSchema = new Map([
  [
    FunderAccount,
    {
      kind: 'struct',
      fields: [
        ['funder', ['u8', 32]],
        ['nft_mint', ['u8', 32]],
        ['tokens_mint', ['u8', 32]],
        ['fund_invested', 'u64'],
        ['lamports_per_token', 'u64'],
      ],
    },
  ],
]);
export const InitAccountSchema = new Map([
  [
    InitAccount,
    {
      kind: 'struct',
      fields: [
        ['bump', 'u8'],
        ['lamports', 'u64'],
        ['size', 'u64'],
      ],
    },
  ],
]);
export const FundRaisingSchema = new Map([
  [
    FundRaising,
    {
      kind: 'struct',
      fields: [
        ['fund_raising', 'u8'],
        ['nft_mint', ['u8', 32]],
        ['tokens_mint', ['u8', 32]],
        ['funds_collected', 'u64'],
        ['number_of_tokens', 'u64'],
        ['lamports_per_token', 'u64'],
        ['bump', 'u8'],
      ],
    },
  ],
]);
export const InitPDASchema = new Map([
  [
    InitPDA,
    {
      kind: 'struct',
      fields: [
        ['init_pda', 'u8'],
      ],
    },
  ],
]);
export const LamportsSchema = new Map([
  [
    Lamports,
    {
      kind: 'struct',
      fields: [
        ['lamports', 'u64'],
      ],
    },
  ],
]);
export const StartVotingSchema = new Map([
  [
    StartVoting,
    {
      kind: 'struct',
      fields: [
        ['offer', 'u64'],
        ['proposer_pda', 'u8'],
        ['vote_account_pda', 'u8'],
      ],
    },
  ],
]);
export const TermsSchema = new Map([
  [
    Terms,
    {
      kind: 'struct',
      fields: [
        ['is_init', 'u8'],
        ['funder_account', 'u64'],
        ['funder_account_size', 'u64'],
        ['fundrasing_account', 'u64'],
        ['fundrasing_account_size', 'u64'],
        ['nft_pda_account', 'u64'],
        ['nft_pda_account_size', 'u64'],
        ['vote_account', 'u64'],
        ['vote_account_size', 'u64'],
        ['proposal_account', 'u64'],
        ['proposal_account_size', 'u64'],
        ['mint', 'u64'],
        ['mint_size', 'u64'],
        ['token_distribution_account', 'u64'],
        ['token_distribution_account_size', 'u64'],
        ['usertokenmint_account','u64'],
        ['usertokenmint_account_size','u64'],
        ['token_to_sol_account','u64'],
        ['token_to_sol_account_size','u64'],
        ['investor_account','u64'],
        ['investor_account_size','u64'],
        ['lamports_per_token_fundraising','u64'],
        ['minimum_lamports_per_token','u64'],
        ['buy_sell_order_account','u64'],
        ['buy_sell_order_account_size','u64'],
        ['small_account','u64'],
      ],
    },
  ],
]);
export const UserAddresTokenMintSchema = new Map([
  [
    UserAddresTokenMint,
    {
      kind: 'struct',
      fields: [
        ['owner', ['u8', 32]],
        ['token_mint', ['u8', 32]],
      ],
    },
  ],
]);

export class SolToken {

  tokenization_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  number_of_tokens:bigint=BigInt(0);
  lamports_per_token:bigint=BigInt(0);
  tokens_sold:bigint=BigInt(0);

  constructor(fields?: {
    tokenization_mint?: number[];
    number_of_tokens?:bigint;
    lamports_per_token?:bigint;
    tokens_sold?:bigint;
  }) {
      if (fields) {
          if (fields.tokenization_mint !== undefined) this.tokenization_mint = fields.tokenization_mint;
          if (fields.number_of_tokens !== undefined) this.number_of_tokens = fields.number_of_tokens;
          if (fields.lamports_per_token !== undefined) this.lamports_per_token = fields.lamports_per_token;
          if (fields.tokens_sold !== undefined) this.tokens_sold = fields.tokens_sold;
      }
  }
}
export const SolTokenSchema = new Map([
  [
    SolToken,
    {
      kind: 'struct',
      fields: [
        ['tokenization_mint', ['u8', 32]],
        ['number_of_tokens', 'u64'],
        ['lamports_per_token', 'u64'],
        ['tokens_sold', 'u64'],
      ],
    },
  ],
]);


export class Proposal {
  proposer: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  nft_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  offer: bigint = BigInt(0);

  constructor(fields?: {
    proposer?: number[];
      nft_mint?: number[];
      offer: bigint;
}) {
      if (fields) {
          if (fields.proposer) this.proposer = fields.proposer;
          if (fields.nft_mint) this.nft_mint = fields.nft_mint;
          if (fields.offer !== undefined) this.offer = fields.offer;
      }
  }
}
export const ProposalSchema = new Map([
  [
    Proposal,
    {
      kind: 'struct',
      fields: [
        ['proposer', ['u8', 32]],
        ['nft_mint', ['u8', 32]],
        ['offer', 'u64'],
      ],
    },
  ],
]);

export class VoteData {

  refuse_accept: number = 0;
  vote_account_pda_bump: number = 0;

  constructor(fields?: {
    refuse_accept?: number;
    vote_account_pda_bump?: number;

}) {
      if (fields) {
          if (fields.refuse_accept) this.refuse_accept = fields.refuse_accept;
          if (fields.vote_account_pda_bump) this.vote_account_pda_bump = fields.vote_account_pda_bump;

      }
  }
}
export const VoteDataSchema = new Map([
  [
    VoteData,
    {
      kind: 'struct',
      fields: [

        ['refuse_accept', 'u8'],
        ['vote_account_pda_bump', 'u8'],
      ],
    },
  ],
]);

export class BuySellToken {
  is_init:number = 0;
  owner: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  token_mint: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,];
  price_per_token: bigint = BigInt(0);
  amount: bigint = BigInt(0);

  constructor(fields?: {
    is_init:number;
    owner?: number[];
    token_mint?: number[];
    price_per_token?: bigint;
    amount?: bigint;
  }) {
      if (fields) {
        if (fields.is_init) this.is_init = fields.is_init;
        if (fields.owner) this.owner = fields.owner;
        if (fields.token_mint) this.token_mint = fields.token_mint;
        if (fields.price_per_token !== undefined) this.price_per_token = fields.price_per_token;
        if (fields.amount !== undefined) this.amount = fields.amount;
      }
  }
}
export class BuySellOrder {
  price_per_token: bigint = BigInt(0);
  amount: bigint = BigInt(0);

  constructor(fields?: {
    price_per_token: bigint;
    amount: bigint;
  }) {
      if (fields) {
          if (fields.price_per_token !== undefined) this.price_per_token = fields.price_per_token;
          if (fields.amount !== undefined) this.amount = fields.amount;
      }
  }
}

export const BuySellOrderSchema = new Map([
  [
    BuySellOrder,
    {
      kind: 'struct',
      fields: [
        ['is_init', 'u8'],
        ['buyerseller', ['u8', 32]],
        ['token_mint', ['u8', 32]],
        ['price_per_token', 'u64'],
        ['amount', 'u64'],
      ],
    },
  ],
]);
export const BuySellTokenSchema = new Map([
  [
    BuySellOrder,
    {
      kind: 'struct',
      fields: [
        ['price_per_token', 'u64'],
        ['amount', 'u64'],
      ],
    },
  ],
]);