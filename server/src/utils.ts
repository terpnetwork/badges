    
import { IPFS_GATEWAY_TEMPLATE, SITE_URL } from './constants'

    // Convert IPFS protocol URL to HTTPS protocol URL using IPFS gateway. source: https://github.com/DA0-DA0/dao-dao-ui/blob/development/packages/utils/conversion.ts#L278C1-L282C14
export const transformIpfsUrlToHttpsIfNecessary = (ipfsUrl: string) =>
ipfsUrl.startsWith('ipfs://')
  ? IPFS_GATEWAY_TEMPLATE.replace('PATH', ipfsUrl.replace('ipfs://', ''))
  : ipfsUrl