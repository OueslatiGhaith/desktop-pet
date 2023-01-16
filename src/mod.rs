use bevy_assets_bundler::AssetBundlingOptions;

lazy_static::lazy_static! {
    pub static ref BUNDLE_OPTIONS: AssetBundlingOptions = AssetBundlingOptions {
        encode_file_names: true,
        enabled_on_debug_build: true,
        ..Default::default()
    };
}
