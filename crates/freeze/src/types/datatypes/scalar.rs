use crate::{datasets::*, define_datatypes, types::columns::ColumnData, ColumnType, *};
use polars::prelude::*;
use std::collections::HashMap;

define_datatypes!(
    BalanceDiffs,
    Balances,
    Blocks,
    CodeDiffs,
    Codes,
    Contracts,
    Erc20Balances,
    Erc20Metadata,
    Erc20Supplies,
    Erc20Transfers,
    Erc721Metadata,
    Erc721Transfers,
    EthCalls,
    Logs,
    NonceDiffs,
    Nonces,
    StorageDiffs,
    Storages,
    Traces,
    TraceCalls,
    Transactions,
    TransactionAddresses,
    VmTraces,
    NativeTransfers,
);

impl Datatype {
    fn alias_map() -> HashMap<String, Datatype> {
        let mut map = HashMap::new();
        for datatype in Datatype::all() {
            let key = datatype.name();
            if map.contains_key(&key) {
                panic!("conflict in datatype names")
            }
            map.insert(key, datatype);
            for key in datatype.aliases().into_iter() {
                if map.contains_key(key) {
                    panic!("conflict in datatype names")
                }
                map.insert(key.to_owned(), datatype);
            }
        }
        map
    }
}

impl std::str::FromStr for Datatype {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Datatype, ParseError> {
        let mut map = Datatype::alias_map();
        map.remove(s)
            .ok_or_else(|| ParseError::ParseError(format!("no datatype matches input: {}", s)))
    }
}
