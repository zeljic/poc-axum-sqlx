fn get_env_var(key: &str, default: Option<&str>) -> String {
	std::env::var(key).unwrap_or_else(|_| {
		default
			.map(|s| s.to_string())
			.unwrap_or_else(|| panic!("{} must be set", key))
	})
}

#[derive(Debug, Clone)]
pub struct Config {
	pub server_host: String,
	pub server_port: u16,

	pub log_level: String,
	pub log_file_path: String,

	pub db_url: String,
	pub db_max_connections: u32,
	pub db_min_connections: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
	pub fn new() -> Self {
		Self {
			server_host: get_env_var("SERVER_HOST", Some("127.0.0.1")),
			server_port: get_env_var("SERVER_PORT", Some("8080")).parse().expect("SERVER_PORT must be a number"),

			log_level: get_env_var("LOG_LEVEL", Some("error")),
			log_file_path: get_env_var("LOG_FILE_PATH", Some("app.log")),

			db_url: get_env_var("DB_URL", None),
			db_max_connections: get_env_var("DB_MAX_CONNECTIONS", Some("4")).parse().expect("DB_MAX_CONNECTIONS must be a number"),
			db_min_connections: get_env_var("DB_MIN_CONNECTIONS", Some("2")).parse().expect("DB_MIN_CONNECTIONS must be a number"),
		}
	}
}