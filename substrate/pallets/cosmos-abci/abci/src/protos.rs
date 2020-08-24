mod libs {
    pub mod kv {
        tonic::include_proto!("tendermint.libs.kv");
    }
}

mod crypto {
    pub mod merkle {
        tonic::include_proto!("tendermint.crypto.merkle");
    }
}

mod proto {
    pub mod abci_proto {
        tonic::include_proto!("tendermint.abci.types");
    }
}

pub use libs::kv::*;
pub use crypto::merkle::*;
pub use proto::abci_proto::*;
