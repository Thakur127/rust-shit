#[derive(Debug, Clone)]
pub struct CacheConfigPathOption {
    pub pathname: String,
    pub ttl: u64,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub paths: Option<Vec<CacheConfigPathOption>>,
}

#[derive(Debug, Clone)]
pub struct ProxyServerConfig {
    pub port: Option<String>,
    pub host: Option<String>,
    pub target_hosts: Option<Vec<String>>,
    pub cache: Option<CacheConfig>,
}
