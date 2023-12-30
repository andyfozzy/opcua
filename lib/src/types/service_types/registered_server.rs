// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2024 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
#[allow(unused_imports)]
use crate::types::{
    basic_types::*, encoding::*, localized_text::LocalizedText, node_ids::ObjectId,
    service_types::enums::ApplicationType, service_types::impls::MessageInfo, string::UAString,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
pub struct RegisteredServer {
    pub server_uri: UAString,
    pub product_uri: UAString,
    pub server_names: Option<Vec<LocalizedText>>,
    pub server_type: ApplicationType,
    pub gateway_server_uri: UAString,
    pub discovery_urls: Option<Vec<UAString>>,
    pub semaphore_file_path: UAString,
    pub is_online: bool,
}

impl MessageInfo for RegisteredServer {
    fn object_id(&self) -> ObjectId {
        ObjectId::RegisteredServer_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RegisteredServer> for RegisteredServer {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.server_uri.byte_len();
        size += self.product_uri.byte_len();
        size += byte_len_array(&self.server_names);
        size += self.server_type.byte_len();
        size += self.gateway_server_uri.byte_len();
        size += byte_len_array(&self.discovery_urls);
        size += self.semaphore_file_path.byte_len();
        size += self.is_online.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.server_uri.encode(stream)?;
        size += self.product_uri.encode(stream)?;
        size += write_array(stream, &self.server_names)?;
        size += self.server_type.encode(stream)?;
        size += self.gateway_server_uri.encode(stream)?;
        size += write_array(stream, &self.discovery_urls)?;
        size += self.semaphore_file_path.encode(stream)?;
        size += self.is_online.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let server_uri = UAString::decode(stream, decoding_options)?;
        let product_uri = UAString::decode(stream, decoding_options)?;
        let server_names: Option<Vec<LocalizedText>> = read_array(stream, decoding_options)?;
        let server_type = ApplicationType::decode(stream, decoding_options)?;
        let gateway_server_uri = UAString::decode(stream, decoding_options)?;
        let discovery_urls: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        let semaphore_file_path = UAString::decode(stream, decoding_options)?;
        let is_online = bool::decode(stream, decoding_options)?;
        Ok(RegisteredServer {
            server_uri,
            product_uri,
            server_names,
            server_type,
            gateway_server_uri,
            discovery_urls,
            semaphore_file_path,
            is_online,
        })
    }
}
