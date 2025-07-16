use std::{env, thread};

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub log_config_path: String,
    pub worker_size: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            log_config_path: "log4rs.yaml".to_string(),
            worker_size: thread::available_parallelism().unwrap().get(),
        }
    }
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap_or(8080);

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let log_config_path = env::var("LOG4RS_CONFIG_PATH").unwrap_or("log4rs.yaml".to_string());

        let cpu_size = thread::available_parallelism().unwrap().get();
        let worker_size = env::var("WORKER_SIZE")
            .unwrap_or_else(|_| cpu_size.to_string())
            .parse::<usize>()
            .unwrap_or(cpu_size)
            .min(cpu_size);

        Self {
            host,
            port,
            log_config_path,
            worker_size,
        }
    }
}

pub fn initialize_logging(config_path: &str) -> Result<(), std::io::Error> {
    log4rs::init_file(config_path, Default::default())
        .map_err(|e| std::io::Error::other(format!("Failed to initialize logging: {e}")))?;
    Ok(())
}
