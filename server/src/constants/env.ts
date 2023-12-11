// Constants derived from environment variables.

// import { ChainId } from '@dao-dao/types'

export const VERCEL_ENV = process.env.NEXT_PUBLIC_VERCEL_ENV

export const IPFS_GATEWAY_TEMPLATE = process.env
  .NEXT_PUBLIC_IPFS_GATEWAY_TEMPLATE as string

export const SITE_URL =
  // On local dev or production vercel, use manually set domain.
  !VERCEL_ENV || VERCEL_ENV === 'production'
    ? (process.env.NEXT_PUBLIC_SITE_URL as string)
    : // Use vercel deployment URL if on preview or development vercel build.
      `https://${process.env.NEXT_PUBLIC_VERCEL_URL}`



// // True if on mainnet, false if on testnet.
// export const MAINNET = process.env.NEXT_PUBLIC_MAINNET === 'true'
// export const DEFAULT_CHAIN_ID = MAINNET
//   ? ChainId.JunoMainnet
//   : ChainId.JunoTestnet

// export const DAO_DAO_DAO_ADDRESS = process.env
//   .NEXT_PUBLIC_DAO_DAO_DAO_ADDRESS as string

// // https://dashboard.web3auth.io
// export const WEB3AUTH_CLIENT_ID = process.env
//   .NEXT_PUBLIC_WEB3AUTH_CLIENT_ID as string

// export const CI = process.env.CI === 'true'

// // Stargaze
// export const STARGAZE_GQL_INDEXER_URI = process.env
//   .NEXT_PUBLIC_STARGAZE_GQL_INDEXER_URI as string
// export const STARGAZE_URL_BASE = process.env
//   .NEXT_PUBLIC_STARGAZE_URL_BASE as string
// export const STARGAZE_NAMES_CONTRACT = process.env
//   .NEXT_PUBLIC_STARGAZE_NAMES_CONTRACT as string

// // Wallet profiles
// export const PFPK_API_BASE = process.env.NEXT_PUBLIC_PFPK_API_BASE as string

// // Indexer
// export const INDEXER_URL = process.env.NEXT_PUBLIC_INDEXER_URL
// // Disables the indexer in place of RPC nodes. Either way, the indexer is still
// // used for the features that depend on it, like the inbox and vote timestamps.
// export const INDEXER_DISABLED =
//   process.env.NEXT_PUBLIC_INDEXER_DISABLED === 'true'

// // Search
// export const SEARCH_HOST = process.env.NEXT_PUBLIC_SEARCH_HOST as string
// export const SEARCH_API_KEY = process.env.NEXT_PUBLIC_SEARCH_API_KEY as string

export const NFT_STORAGE_API_KEY = process.env.NFT_STORAGE_API_KEY as string

// export const FAST_AVERAGE_COLOR_API_TEMPLATE = process.env
//   .NEXT_PUBLIC_FAST_AVERAGE_COLOR_API_TEMPLATE as string

export const DISABLED_ACTIONS = (
  process.env.NEXT_PUBLIC_DISABLED_ACTIONS || ''
).split(',')

// // Discord notifier (https://github.com/DA0-DA0/discord-notifier-cf-worker)
// export const DISCORD_NOTIFIER_CLIENT_ID = process.env
//   .NEXT_PUBLIC_DISCORD_NOTIFIER_CLIENT_ID as string
// export const DISCORD_NOTIFIER_API_BASE = process.env
//   .NEXT_PUBLIC_DISCORD_NOTIFIER_API_BASE as string
// export const DISCORD_NOTIFIER_REDIRECT_URI = SITE_URL + '/discord'

// // Inbox API (https://github.com/DA0-DA0/inbox-cf-worker)
// export const INBOX_API_BASE = process.env.NEXT_PUBLIC_INBOX_API_BASE as string

// // KVPK API (https://github.com/DA0-DA0/kvpk)
// export const KVPK_API_BASE = process.env.NEXT_PUBLIC_KVPK_API_BASE as string

// // Single DAO Mode
// export const SINGLE_DAO_MODE =
//   process.env.NEXT_PUBLIC_SINGLE_DAO_MODE === 'true'

// // Kado API (https://docs.kado.money)
// export const KADO_API_KEY = process.env.NEXT_PUBLIC_KADO_API_KEY as string

// // WYND
// export const WYND_MULTI_HOP_CONTRACT = process.env
//   .NEXT_PUBLIC_WYND_MULTI_HOP_CONTRACT as string
// export const WYND_API_BASE = process.env.NEXT_PUBLIC_WYND_API_BASE as string
// export const WYND_REFERRAL_COMMISSION = Number(
//   process.env.NEXT_PUBLIC_WYND_REFERRAL_COMMISSION || '0.01'
// )

// WebSockets API
export const WEB_SOCKET_PUSHER_APP_KEY = process.env
  .NEXT_PUBLIC_WEB_SOCKET_PUSHER_APP_KEY as string
export const WEB_SOCKET_PUSHER_HOST = process.env
  .NEXT_PUBLIC_WEB_SOCKET_PUSHER_HOST as string
export const WEB_SOCKET_PUSHER_PORT = Number(
  process.env.NEXT_PUBLIC_WEB_SOCKET_PUSHER_PORT || '6001'
)

// Web Push
export const WEB_PUSH_PUBLIC_KEY = process.env
  .NEXT_PUBLIC_WEB_PUSH_PUBLIC_KEY as string