mod src;
use src::BUNDLE_OPTIONS;

use bevy_assets_bundler::*;

/// bundles the assets into a single file
fn main() {
    AssetBundler::from(BUNDLE_OPTIONS.clone()).build().unwrap();
}
