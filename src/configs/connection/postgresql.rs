use serde::{Deserialize, Serialize};

use crate::traits::PreflightCheck;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgreSQLConnection {
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
}

impl PostgreSQLConnection {
    pub fn parse(&self) -> PostgreSQLConnection {
        PostgreSQLConnection {
            host: Self::from_env_or_default(self.host.clone()),
            port: Self::from_env_or_default(self.port.clone()),
            user: Self::from_env_or_default(self.user.clone()),
            password: Self::from_env_or_default(self.password.clone()),
        }
    }

    fn from_env_or_default(val: String) -> String {
        if val.starts_with("env=") {
            let env_var = val.split('=').collect::<Vec<&str>>()[1];
            match std::env::var(env_var) {
                Ok(val) => val,
                Err(_) => panic!("Environment variable {} not found", env_var),
            }
        } else {
            val
        }
    }
}

impl PreflightCheck for PostgreSQLConnection {
    async fn preflight_check(&self) -> Result<(), String> {
        let conn = self.parse();

        check_empty(conn.host, "Host is empty")?;
        check_empty(conn.port, "Port is empty")?;
        check_empty(conn.user, "User is empty")?;
        check_empty(conn.password, "Password is empty")?;

        Ok(())
    }
}

fn check_empty(field: String, error_message: &str) -> Result<(), String> {
    if field.is_empty() {
        return Err(error_message.to_string());
    }
    Ok(())
}
