mod api_models;
mod cached_client;
mod client;
mod connect_player;

pub mod cache;

pub use cached_client::{CachedSpotifyClient, SpotifyApiClient, SpotifyResult};
pub use client::SpotifyApiError;
pub use connect_player::SpotifyConnectPlayer;

pub async fn clear_user_cache() -> Option<()> {
    cache::CacheManager::for_dir("spot/net")?
        .clear_cache_pattern(&*cached_client::USER_CACHE)
        .await
        .ok()
}
