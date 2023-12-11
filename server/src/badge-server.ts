 

 // step 1: wait for tea-id 
 // step 2: query hub contract for Tea, expect response = TeaResponse
 // step 3: return metadata cid string from response.metadata, into {serial}
 // step 4: request response from ipfs gateway with `${api_url}?id=${id}&serial=${serial}`
 // step 5: return tea metadata in json 
