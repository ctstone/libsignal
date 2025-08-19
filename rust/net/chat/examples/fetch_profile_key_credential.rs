//
// Copyright 2025 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use std::time::SystemTime;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::{Parser, ValueEnum};
use libsignal_cli_utils::args::{parse_aci, parse_hex_bytes};
use libsignal_core::Aci;
use libsignal_net::chat::test_support::simple_chat_connection;
use libsignal_net::infra::EnableDomainFronting;
use libsignal_net_chat::api::profiles::UnauthenticatedChatApi as _;
use libsignal_net_chat::api::{Unauth, UserBasedAuthorization};
use zkgroup::profiles::ProfileKey;

#[derive(Parser)]
struct Config {
    env: Environment,
    #[arg(value_parser=parse_aci)]
    aci: Aci,
    #[arg(value_parser=parse_hex_bytes::<32>)]
    profile_key: [u8; zkgroup::PROFILE_KEY_LEN],
}

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Environment {
    Staging,
    #[value(alias("prod"))]
    Production,
}

const ZKGROUP_PARAMS_STAGING: &str = "ABSY21VckQcbSXVNCGRYJcfWHiAMZmpTtTELcDmxgdFbtp/bWsSxZdMKzfCp8rvIs8ocCU3B37fT3r4Mi5qAemeGeR2X+/YmOGR5ofui7tD5mDQfstAI9i+4WpMtIe8KC3wU5w3Inq3uNWVmoGtpKndsNfwJrCg0Hd9zmObhypUnSkfYn2ooMOOnBpfdanRtrvetZUayDMSC5iSRcXKpdlukrpzzsCIvEwjwQlJYVPOQPj4V0F4UXXBdHSLK05uoPBCQG8G9rYIGedYsClJXnbrgGYG3eMTG5hnx4X4ntARBgELuMWWUEEfSK0mjXg+/2lPmWcTZWR9nkqgQQP0tbzuiPm74H2wMO4u1Wafe+UwyIlIT9L7KLS19Aw8r4sPrXZSSsOZ6s7M1+rTJN0bI5CKY2PX29y5Ok3jSWufIKcgKOnWoP67d5b2du2ZVJjpjfibNIHbT/cegy/sBLoFwtHogVYUewANUAXIaMPyCLRArsKhfJ5wBtTminG/PAvuBdJ70Z/bXVPf8TVsR292zQ65xwvWTejROW6AZX6aqucUjlENAErBme1YHmOSpU6tr6doJ66dPzVAWIanmO/5mgjNEDeK7DDqQdB1xd03HT2Qs2TxY3kCK8aAb/0iM0HQiXjxZ9HIgYhbtvGEnDKW5ILSUydqH/KBhW4Pb0jZWnqN/YgbWDKeJxnDbYcUob5ZY5Lt5ZCMKuaGUvCJRrCtuugSMaqjowCGRempsDdJEt+cMaalhZ6gczklJB/IbdwENW9KeVFPoFNFzhxWUIS5ML9riVYhAtE6JE5jX0xiHNVIIPthb458cfA8daR0nYfYAUKogQArm0iBezOO+mPk5vCNWI+wwkyFCqNDXz/qxl1gAntuCJtSfq9OC3NkdhQlgYQ==";
const ZKGROUP_PARAMS_PROD: &str = "AAp8oB0D4EV2q7hSue3Kxzh1Vc88/nmLuRR9G3EefC0+CMcxJFQwDMgjFvFBKx3o6m9gJLevYiKcm/NxXX9WtnFMDHgDgfqHxbCi2rm20SgoHnuoph6XArmEOX6a1xLJVxgDtgfm1IbcyyqROXYxe9v2RvMUAnjbLI/fm0rXXhldjszlVR/wRpybX90RUjFyL/2Achttf3IC/ShWKkB6mWXwuFCcNfzeCCQ+w7cNnDbWscBcrhuou7HZvbt16/YdCXLyp+WdwS8ZkelpITvyK2hsPvf4oxaRLQfVRYXUMX55xpapbKH6PthuOzMVRkf+I3Xz3/bNjiQSlQkmAXlgB1YujgABYnJ6yJXQKP2mR4UJ3UYoGroYoafWycDa+vUYYozaUmzFjsBYWpYE+HyPJlJ2QaFTrpVqxX7NXsSbg8t35IvfWfZME9YBZ2eErDunwkaE4iDQhHl5IXAhbHDrr2QaJ68YIkn7lJSgFDKGFB2kb6BvDUGzcpI/CTHQi6WlCqQidQLJWDFFdlYjrUCQM2vvJtgyGrSc89jdXTFjM31aqmtcPWgWL0qv+RmK/BC392Nsu8WoSJcAE4yhccQuRSemtolgwewnjasoOFBNOPh4+pX55SwhyTVgtwl+NTNVNFydxGp9Me8ogRWElzwA9BFtNAgQtlfgIyZRTetFqLkYmIBDxwMcpizDKES5lPhV2uJJuzcMq/06mVQz2OrXgglWk01uN8U59pfNFpTZhcGQv+MHjwEAudq5eLpt3aFrdxJ7D26Fwl5j215SJ0yZo7vmSEML1vf7FaGh0IL57bRpCvdebB5WapSChUX+PPvCXohVjGrERFvQpeET6pydGGlEKYLWuWa3zFGmPvJJYZ/QfcmIP9zyhqzQT/7a7RIqFA==";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_module(module_path!(), log::LevelFilter::Info)
        .parse_default_env()
        .init();

    let Config {
        env,
        aci,
        profile_key,
    } = Config::parse();
    let (env, zkparams) = match env {
        Environment::Staging => (libsignal_net::env::STAGING, ZKGROUP_PARAMS_STAGING),
        Environment::Production => (libsignal_net::env::PROD, ZKGROUP_PARAMS_PROD),
    };

    let chat_connection = Unauth(
        simple_chat_connection(&env, EnableDomainFronting::AllDomains, None, |_route| true).await?,
    );

    let zkparams: zkgroup::ServerPublicParams =
        zkgroup::deserialize(&BASE64_STANDARD.decode(zkparams).unwrap()).unwrap();
    let profile_key = ProfileKey::create(profile_key);
    let request_context =
        zkparams.create_profile_key_credential_request_context(rand::random(), aci, profile_key);

    let response = chat_connection
        .get_profile_key_credential(
            aci,
            profile_key,
            request_context.get_request(),
            UserBasedAuthorization::AccessKey(profile_key.derive_access_key()),
        )
        .await?;
    _ = zkparams.receive_expiring_profile_key_credential(
        &request_context,
        &response,
        SystemTime::now().into(),
    )?;

    log::info!("success!");
    Ok(())
}
