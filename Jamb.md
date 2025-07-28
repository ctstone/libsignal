## Generate Key transparency keys:

```
# TODO: are these valid? Found in https://github.com/signalapp/key-transparency-server/tree/main/cmd/generate-keys

# Generate Ed25519 private key
openssl genpkey -algorithm Ed25519 -out keytrans_signing.pem

# Extract public key
openssl pkey -in keytrans_signing.pem -pubout -out keytrans_public.pem

# Convert to raw bytes (32 bytes each)
openssl pkey -in keytrans_signing.pem -raw -noout | xxd -p -c 32
openssl pkey -in keytrans_public.pem -pubin -raw -noout | xxd -p -c 32


```

## Update env

```
# IPv4
dig A jamb-dev-signal-server.ngrok.app

# IPv6
dig AAAA jamb-dev-signal-server.ngrok.app
```

Source: `rust/net/src/env.rs`

```
// Update IPv4, IPv6 and hostname:
const DOMAIN_CONFIG_CHAT: DomainConfig = DomainConfig {
    ip_v4: &[
        ip_addr!(v4, "3.13.191.225"), // <-- IPv4 (1)
        ip_addr!(v4, "3.134.39.220"), // <-- IPv4 (2)
    ],
    ip_v6: &[
        ip_addr!(v6, "2600:1f16:d83:1202::6e:5"), // <-- IPv6 (1)
        ip_addr!(v6, "2600:1f16:d83:1201::6e:1"), // <-- IPv6 (2)
    ],
    connect: ConnectionConfig {
        hostname: "jamb-dev-signal-server.ngrok.app", // <-- custom host
        port: DEFAULT_HTTPS_PORT,
        cert: SIGNAL_ROOT_CERTIFICATES, // <-- instead use RootCertificates::Native
        min_tls_version: Some(SslVersion::TLS1_3),
        confirmation_header_name: Some(TIMESTAMP_HEADER_NAME),
        proxy: Some(ConnectionProxyConfig {
            path_prefix: "/service",
            configs: [PROXY_CONFIG_F_PROD, PROXY_CONFIG_G],
        }),
    },
};

// TBD: need our own keys!
pub(crate) const KEYTRANS_SIGNING_KEY_MATERIAL_PROD: &[u8; 32] =
    &hex!("a3973067984382cfa89ec26d7cc176680aefe92b3d2eba85159dad0b8354b622");
pub(crate) const KEYTRANS_VRF_KEY_MATERIAL_PROD: &[u8; 32] =
    &hex!("3849cf116c7bc9aef5f13f0c61a7c246e5bade4eb7e1c7b0efcacdd8c1e6a6ff");
pub(crate) const KEYTRANS_AUDITOR_KEY_MATERIAL_PROD: &[u8; 32] =
    &hex!("2d973608e909a09e12cbdbd21ad58775fd72fe1034a5a079f26541d5764ce17f");
```