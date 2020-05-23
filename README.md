# rust-sgx-remote-attestation
Remote attestation framework for Fortanix EDP
## Disclaimer
This project is highly experimental at the current stage, so I would advise aginst using it in production. I will keep updating the code and adding more instructions.
## Dependencies
- [Fortanix EDP](https://edp.fortanix.com/docs/installation/guide/)
- [Clang 3.8 or older](https://releases.llvm.org/download.html) for building [rust-mbedtls](https://github.com/fortanix/rust-mbedtls)

Note: If your system has a newer version of Clang installed, try pointing clang-sys to a different version of Clang by exporting [these environment variables](https://github.com/KyleMayes/clang-sys#environment-variables), including `LD_LIBRARY_PATH`, before building. For example,
```bash
export LLVM_CONFIG_PATH=/real/path/to/clang/bin/llvm-config
export LD_LIBRARY_PATH=/real/path/to/clang/lib
```
before building and running (see below).
## How to Build and Run Example
1. Sign up for a Development Access account at https://api.portal.trustedservices.intel.com/EPID-attestation. Make sure that the Name Base Mode is Linkable Quote (this is all the SDK can support for now). Take note of "SPID", "Primary key", and "Secondary key".
2. Modify the following fields in [settings.json](ra-sp/examples/data/settings.json) using the information from the previous step:
  - "spid": "\<SPID\>"
  - "primary_subscription_key": "\<Primary Key\>"
  - "secondary_subscription_key": "\<Secondary key\>"
3. Download IAS's root certificate from [this link](https://certificates.trustedservices.intel.com/Intel_SGX_Attestation_RootCA.pem) and save the cerficate file in directory [sample-sp/data](ra-sp/examples/data). Make sure the file name is "Intel_SGX_Attestation_RootCA.pem".
4. Run the script `build.sh` and `run.sh` consecutively from the main directory.

If there are no error messages on the screen, then the remote attestation has run successfully.

## Customization
- [settings.json](ra-sp/examples/data/settings.json)
  - `linkable`:  `true` for Linkable Quotes or `false` for Unlinkable Quotes. Only Linkable Quotes are currently supported.
  - `random_nonce`: `true` for using Quote nonce or `false` otherwise. Quote nonce is currently unsupported. 
  - `use_platform_service`: `true` to use Platform Service Enclave (PSE) or `false` otherwise. PSE is currently unsupported.
  - `spid`: Service Provider ID to talk to IAS.
  - `primary_subscription_key`: subscription key to talk to IAS.
  - `secondary_subscription_key`: subscription key to talk to IAS.
  - `quote_trust_options`: list of quote status options to be accepted as successful remote attestation. If left empty, quote status must be `"OK"` to be considered successful. For all the options, check the [API documentation](https://api.trustedservices.intel.com/documents/sgx-attestation-api-spec.pdf) and look for "isvEnclaveQuoteStatus".
  - `sp_private_key_pem_path`: path to SP's private key file in PEM format. This is used for authentication during key-exchange so it must be regenerated and kept secret.
  - `ias_root_cert_pem_path`: path to IAS root certificate for SP to verify IAS during attestation. This can be downloaded from [this link](https://certificates.trustedservices.intel.com/Intel_SGX_Attestation_RootCA.pem).
  - `sigstruct_path`: path to enclave's SIGSTRUCT generated by the `sgxs-sign` command. This is provided by the vendor, i.e. the party who builds and signs enclave files, to SP for SP to verify enclave authenticity.

- [sp_vkey.rs](ra-enclave/examples/sp_vkey.rs), [sp-keys/public_key.pem](ra-sp/examples/data/sp-keys/public_key.pem) and [sp-keys/private_key.pem](ra-sp/examples/data/sp-keys/private_key.pem): 
SP's signing and verification key. This key pair is used to provide authentication for SP to the enclave during key-exchange, and therefore must be fresh for every SP. `private_key.pem` must be kept secret by SP. `sp_vkey.rs` containts the same key as in `public_key.pem`, but `sp_vkey.rs` must be embeded in the enclave file at compile time, i.e. it must not be read into the enclave from the file system at runtime, in order to prevent man-in-the-middle attacks. 

- [vendor-keys/private_key.pem](ra-enclave/examples/data/vendor-keys/private_key.pem): Vendor's signing key. This is used by software vendors to sign enclave files with the `sgxs-sign` command. This signing key must be regenerated.

## TODO
- Update `aesm_client` to version `0.4` to support Quote nonce and Linkable Quotes. See [this commit](https://github.com/fortanix/rust-sgx/commit/bd5fa092b93248fd36a707fd406ac8b72e6e8692#diff-50494cfb8392ff712e2ab04a305cf14f).
- Use the secondary subscription key if the primary one fails.
- Support PSE (this doesn't seem to be supported by the main library yet.)
