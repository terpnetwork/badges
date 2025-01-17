use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

use tea::nft::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, CollectionInfoResponse,
    ContractInfoResponse, ExecuteMsg, InstantiateMsg, MinterResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, QueryMsg, TokensResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);

    export_schema(&schema_for!(ContractInfoResponse), &out_dir);
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(ApprovalResponse), &out_dir);
    export_schema(&schema_for!(ApprovalsResponse), &out_dir);
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
    export_schema(&schema_for!(MinterResponse), &out_dir);
    export_schema(&schema_for!(CollectionInfoResponse), &out_dir);

    // types with generics need to be renamed
    export_schema_with_title(
        &schema_for!(ExecuteMsg),
        &out_dir,
        "ExecuteMsg",
    );
    export_schema_with_title(
        &schema_for!(NftInfoResponse),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse),
        &out_dir,
        "AllNftInfoResponse",
    );

    // response types shared by multiple queries need to be renamed
    export_schema_with_title(
        &schema_for!(OperatorsResponse),
        &out_dir,
        "AllOperatorsResponse",
    );
    export_schema_with_title(
        &schema_for!(TokensResponse),
        &out_dir,
        "AllTokensResponse",
    );
}
