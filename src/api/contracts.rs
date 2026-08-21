// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Contract discovery.

use serde::{Deserialize, Serialize};

use crate::{Client, ContractId, ContractMaturityId, Error, Symbol};

const FIND_CONTRACT: &str = "/contract/find";

/// Minimal provider-native contract identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Contract {
    /// Provider contract entity identifier.
    pub id: ContractId,
    /// Provider contract symbol.
    pub name: Symbol,
    /// Owning contract-maturity entity.
    pub contract_maturity_id: ContractMaturityId,
}

#[derive(Serialize)]
struct FindContractQuery<'a> {
    name: &'a str,
}

impl Client {
    /// Finds a contract by its exact provider symbol.
    ///
    /// # Errors
    ///
    /// Returns a typed authentication, transport, provider, bound, or decoding
    /// failure.
    pub async fn find_contract(&self, symbol: &Symbol) -> Result<Contract, Error> {
        self.get(
            FIND_CONTRACT,
            &FindContractQuery {
                name: symbol.as_str(),
            },
        )
        .await
    }
}
