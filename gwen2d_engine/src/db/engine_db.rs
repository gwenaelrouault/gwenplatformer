use log::{debug, error, info};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteQueryResult};
use sqlx::{Error, Pool, Sqlite, SqlitePool};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

static CATEGORY_TABLE: &str = "category";
static ENTITY_TABLE: &str = "entity";
static LEVEL_TABLE: &str = "level";
static FRAME_TABLE: &str = "frame";
static STATE_TABLE: &str = "state";

pub struct EngineDb {
    name: Arc<Mutex<String>>,
    db : Option<Pool<Sqlite>>
}

impl EngineDb {
    pub fn new() -> Self {
        EngineDb {
            name: Arc::new(Mutex::new(String::from(""))),
            db : None
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.db.is_some()
    }

    pub async fn open(&mut self, db_path: &PathBuf) {
        let url = db_path
            .with_extension("db")
            .to_string_lossy()
            .replace('\\', "/");
        let conn_str = format!("{}", url);
        info!("DB:open {}", conn_str);
        let options = SqliteConnectOptions::from_str(&conn_str).unwrap().create_if_missing(true);
        match SqlitePoolOptions::new().connect_with(options).await {
            Ok(pool) => {
                info!("DB:{} open succes", conn_str);
                *self.name.lock().unwrap() = conn_str.to_string();
                self.db = Some(pool.clone());
            }
            Err(e) => {
                error!("DB:{} open failed : {}", conn_str, e);
                *self.name.lock().unwrap() = format!("❌ Erreur : {}", e);
            }
        }
    }
}

async fn create_tables(pool: &Pool<Sqlite>) -> Result<(), Error> {
    let mut tx = pool.begin().await?;
    let create_table_categories = r#"
                CREATE TABLE IF NOT EXISTS category (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    width INTEGER,
                    height INTEGER,
                );
            "#;
    create_table(pool, CATEGORY_TABLE, create_table_categories).await?;
    let create_table_entities = r#"
                CREATE TABLE IF NOT EXISTS entity (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    category_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    width INTEGER,
                    height INTEGER,
                    FOREIGN KEY (category_id) REFERENCES category(id)
                );
            "#;
    create_table(pool, ENTITY_TABLE, create_table_entities).await?;
    let create_table_states = r#"
                CREATE TABLE IF NOT EXISTS states (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    entity_id INTEGER NOT NULL,
                    name TEXT NOT NULL,
                    FOREIGN KEY (entity_id) REFERENCES entity(id)
                );
            "#;
    create_table(pool, STATE_TABLE, create_table_states).await?;
    let create_table_frames = r#"
                CREATE TABLE IF NOT EXISTS frame (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    state_id INTEGER NOT NULL,
                    img BLOB,
                    FOREIGN KEY (state_id) REFERENCES states(id),
                );
            "#;
    create_table(pool, FRAME_TABLE, create_table_frames).await?;;
    let create_table_levels = r#"
                CREATE TABLE IF NOT EXISTS level (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                );
            "#;
    create_table(pool, LEVEL_TABLE, create_table_levels).await?;
    tx.commit().await?;
    Ok(())
}

async fn create_table(pool: &Pool<Sqlite>, name : &str, query : &str) -> Result<(), Error> {
    info!("DB:Creation de la table {}", name);
    sqlx::query(query).execute(pool).await?;
    Ok(())
}
