// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::{
    schema::multikey_layouts::{self},
    utils::util::standardize_address,
};
use anyhow::Result;
use aptos_protos::transaction::v1::{
    account_signature::Signature as AccountSignatureEnum,
    any_public_key::Type as AnyPublicKeyTypeEnumPb, signature::Signature as SignatureEnum,
    MultiKeySignature as MultiKeySignaturePb, Signature as TransactionSignaturePb,
};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(transaction_version, public_key_index))]
#[diesel(table_name = multikey_layouts)]
pub struct MultiKeyLayout {
    pub transaction_version: i64,
    pub transaction_block_height: i64,
    pub signer: String,
    pub type_: String,
    pub public_key: String,
    pub public_key_index: i64,
    pub threshold: i64,
    pub total_public_keys: i64,
    pub is_used: bool,
}

impl MultiKeyLayout {
    pub fn from_user_transaction(
        s: &TransactionSignaturePb,
        sender: &str,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Result<Vec<Self>> {
        match s.signature.as_ref().unwrap() {
            SignatureEnum::SingleSender(s) => {
                let signature = s.sender.as_ref().unwrap();
                match signature.signature.as_ref() {
                    Some(AccountSignatureEnum::MultiKeySignature(s)) => {
                        Ok(Self::parse_multikey_layout(
                            s,
                            sender,
                            transaction_version,
                            transaction_block_height,
                        ))
                    },
                    _ => Ok(vec![]),
                }
            },
            _ => Ok(vec![]),
        }
    }

    fn parse_multikey_layout(
        s: &MultiKeySignaturePb,
        sender: &str,
        transaction_version: i64,
        transaction_block_height: i64,
    ) -> Vec<Self> {
        let signer = standardize_address(sender);
        let mut multikey_layout = Vec::default();
        let total_public_keys = s.public_keys.len() as i64;
        let public_key_indices: HashSet<usize> =
            s.signatures.iter().map(|key| key.index as usize).collect();

        for (index, pk) in s.public_keys.iter().enumerate() {
            let type_ = match AnyPublicKeyTypeEnumPb::try_from(pk.r#type) {
                Ok(AnyPublicKeyTypeEnumPb::Ed25519) => String::from("ed25519"),
                Ok(AnyPublicKeyTypeEnumPb::Secp256k1Ecdsa) => String::from("secp256k1_ecdsa"),
                Ok(AnyPublicKeyTypeEnumPb::Secp256r1Ecdsa) => String::from("secp256r1_ecdsa"),
                Ok(AnyPublicKeyTypeEnumPb::Keyless) => String::from("keyless"),
                _ => String::from("unspecified"),
            };

            multikey_layout.push(Self {
                transaction_version,
                transaction_block_height,
                signer: signer.clone(),
                type_,
                public_key: format!("0x{}", hex::encode(pk.public_key.as_slice())),
                public_key_index: index as i64,
                threshold: s.signatures_required as i64,
                total_public_keys,
                is_used: public_key_indices.contains(&index),
            })
        }

        multikey_layout
    }
}
