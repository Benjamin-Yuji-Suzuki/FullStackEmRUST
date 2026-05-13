use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use sqlx::PgPool;
    use std::sync::OnceLock;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SystemInfo {
        pub id: String,
        pub name: String,
        pub description: Option<String>,
        pub defuzz_method: String,
        pub created_at: String,
    }

    static POOL: OnceLock<PgPool> = OnceLock::new();

    pub fn init_pool(pool: PgPool) {
        let _ = POOL.set(pool);
    }

    pub fn get_pool() -> &'static PgPool {
        POOL.get().expect("Pool not initialized")
    }

    pub async fn list_systems_db() -> Result<Vec<SystemInfo>, String> {
        let pool = get_pool();

        let rows = sqlx::query_as::<_, (String, String, Option<String>, String, String)>(
            "SELECT id::text, name, description, defuzz_method, created_at::text FROM fuzzy_systems ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(rows.into_iter().map(|(id, name, desc, defuzz, created)| SystemInfo {
            id, name, description: desc, defuzz_method: defuzz, created_at: created,
        }).collect())
    }
}}
