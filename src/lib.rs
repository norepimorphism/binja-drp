use binaryninja as binja;
use discord_rich_presence::{
    DiscordIpc as _,
    DiscordIpcClient,
    activity::Activity as DiscordActivity,
    activity::Assets as DiscordAssets,
};

#[no_mangle]
pub extern "C" fn CorePluginInit() -> bool {
    const CLIENT_ID: &str = "989560044339208262";

    let _ = binja::logger::init(log::LevelFilter::Trace);

    log::trace!("Constructing a Discord IPC client...");
    let mut client = DiscordIpcClient::new(CLIENT_ID)
        .expect("failed to construct a Discord IPC client");
    log::trace!("Connecting to Discord...");
    client.connect().expect("failed to connect to Discord");

    log::trace!("Setting Discord activity...");
    if client.set_activity({
        DiscordActivity::new()
            .assets({
                DiscordAssets::new()
                    .large_text("Binary Ninja")
                    .large_image("binja-logo")
                    .small_text("Binary Ninja")
            })
            .details("TODO")
    }).is_err() {
        log::error!("Failed to set Discord activity");
    }
    let _ = client.close();

    // Success.
    true
}
