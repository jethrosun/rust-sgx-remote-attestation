use std::io::Write;
use std::mem::size_of;
use serde::{Serialize, Deserialize};
use serde_big_array::big_array;
use byteorder::{WriteBytesExt, LittleEndian};
use sgx_crypto::signature::Signature;
use sgx_crypto::key_exchange::DHKEPublicKey;
use sgx_crypto::cmac::{Cmac, MacTag, MacError};

pub type Gid = [u8; 4];
pub type Spid = [u8; 16];
pub type PsSecPropDesc = [u8; 256];
pub type Quote = [u8; 1116]; // 436 + quote.signature_len for version 2

big_array! { 
    BigArray; 
    +size_of::<DHKEPublicKey>(), size_of::<Quote>(),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaMsg0 {
    pub exgid: u32,
}


#[derive(Serialize, Deserialize)]
pub struct RaMsg1 {
    pub gid: Gid,
    #[serde(with = "BigArray")]
    pub g_a: DHKEPublicKey 
}

#[derive(Serialize, Deserialize)]
pub struct RaMsg2 {
    #[serde(with = "BigArray")]
    pub g_b: DHKEPublicKey,
    pub spid: Spid,
    pub quote_type: u16, /* unlinkable Quote(0) or linkable Quote(1) */
    pub sign_gb_ga: Signature, 
    pub mac: MacTag, 
    pub sig_rl: Option<Vec<u8>>,
}

impl RaMsg2 {
    pub fn new(smk: &Cmac, 
               g_b: DHKEPublicKey, 
               spid: Spid, 
               quote_type: u16,
               sign_gb_ga: Signature, 
               sig_rl: Option<Vec<u8>>) -> Self {
        let mut msg2 = Self {
            g_b,
            spid,
            quote_type,
            sign_gb_ga,
            mac: [0u8; size_of::<MacTag>()],
            sig_rl,
        };
        let a = msg2.get_a();
        msg2.mac = smk.sign(&a[..]);
        msg2
    }

    pub fn verify_mac(&self, smk: &Cmac) -> Result<(), MacError>{
        let a = self.get_a();
        smk.verify(&a[..], &self.mac)
    }

    fn get_a(&self) -> Vec<u8> {
        let mut a = Vec::new();
        a.write_all(&self.g_b[..]).unwrap();
        a.write_all(&self.spid[..]).unwrap();
        a.write_u16::<LittleEndian>(self.quote_type).unwrap();
        a.write_all(&self.sign_gb_ga[..]).unwrap();
        a
    }
}

#[derive(Serialize, Deserialize)]
pub struct PsSecPropDescInternal {
    #[serde(with = "BigArray")]
    pub inner: PsSecPropDesc,
}

// According to Intel's RA protocol, g_a is needed in MSG3. However, I think this is 
// redundant as SHA256(g_a || g_b || vk) will be included in the report data section of
// Quote anyway. A man-in-the-middle attack should fail if g_a is not signed by QE.
#[derive(Serialize, Deserialize)]
pub struct RaMsg3 {
    pub mac: MacTag,
    pub ps_sec_prop: Option<PsSecPropDescInternal>,
    #[serde(with = "BigArray")]
    pub quote: Quote,
}

impl RaMsg3 {
    pub fn new(smk: &Cmac,
               ps_sec_prop: Option<PsSecPropDesc>,
               quote: Quote) -> Self {
        let ps_sec_prop = ps_sec_prop.map(|v| PsSecPropDescInternal{ inner: v });
        let mut msg3 = Self {
            mac: [0u8; size_of::<MacTag>()],
            ps_sec_prop,
            quote,
        };
        let m = msg3.get_m();
        msg3.mac = smk.sign(&m[..]);
        msg3
    }

    pub fn verify_mac(&self, smk: &Cmac) -> Result<(), MacError> {
        let m = self.get_m();
        smk.verify(&m[..], &self.mac)
    }

    fn get_m(&self) -> Vec<u8> {
        let mut m = Vec::new();
        if self.ps_sec_prop.is_some() {
            m.write_all(&self.ps_sec_prop.as_ref().unwrap().inner[..]).unwrap();
        }
        m.write_all(&self.quote[..]).unwrap();
        m
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RaMsg4 {
    pub is_enclave_trusted: bool,
    pub is_pse_manifest_trusted: Option<bool>,
    pub pib: Option<String>,
}
