use serde_types::hash::H520;
use crypto::Signature;
use libproto::blockchain::{Proof, ProofType};
use rustc_serialize::hex::ToHex;
use bincode::{serialize, deserialize, Infinite};
use std::fmt;
use util::hash::H520 as EthH520;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AuthorityRoundProof {
    pub signature: H520,
    pub step: u64,
}

impl AuthorityRoundProof {
    pub fn new(step: u64, signature: Signature) -> AuthorityRoundProof {
        AuthorityRoundProof {
            step: step,
            signature: EthH520::from(signature.0).into(),
        }
    }
}

impl From<Proof> for AuthorityRoundProof {
    fn from(p: Proof) -> Self {
        let decoded: AuthorityRoundProof = deserialize(&p.get_content()[..]).unwrap();
        decoded
    }
}

impl Into<Proof> for AuthorityRoundProof {
    fn into(self) -> Proof {
        let mut proof = Proof::new();
        let encoded_proof: Vec<u8> = serialize(&self, Infinite).unwrap();
        proof.set_content(encoded_proof);
        proof.set_field_type(ProofType::AuthorityRound);
        proof
    }
}

impl fmt::Display for AuthorityRoundProof {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f,
               "step: {}, signature: {}",
               self.step,
               self.signature.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::{Signature, AuthorityRoundProof};
    use libproto::blockchain::Proof;

    #[test]
    fn proof_display() {
        let proof = AuthorityRoundProof::new(0, Signature::default());
        let string = format!("{}", proof);
        assert_eq!(string,
                   "step: 0, signature: 0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn proof_convert() {
        let o_proof = AuthorityRoundProof::new(0, Signature::default());
        let proto_proof: Proof = o_proof.clone().into();
        let de_proof: AuthorityRoundProof = proto_proof.into();
        assert_eq!(o_proof, de_proof);
    }
}