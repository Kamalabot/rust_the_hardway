#[allow(dead_code)]
// The default runtime flavor is `multi_thread`, but the `rt-multi-thread` feature is disabled.
// adding the rt-multi-thread feature
// https://stackoverflow.com/questions/66960947/what-is-the-smallest-feature-set-to-enable-polling-a-future-with-tokio
use sqlx::{migrate::MigrateDatabase, Row, FromRow, Sqlite, SqlitePool};

// this is the database conn url
const DB_URL: &str = "sqlite://sqlite.db";

#[derive(Clone, Debug, FromRow,)]
struct User {
    id: i64,
    name: String
}

#[derive(Clone, Debug, FromRow,)]
struct Account {
    id: i64,
    name: String,
    active: bool 
}

#[tokio::main]
async fn main() {
    // if the database exists returns True, then database is 
    // not existing, look at the !
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false){
        println!("Creating database: {}", DB_URL);
        // simple match on the create_database trait
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Got the database created..."), 
            Err(err) => println!("{}", err)
        }
    } else {
        println!("Database already exists...");
    }
    // connecting with the database, and panics if fails 
    let newdb = SqlitePool::connect(DB_URL).await.unwrap();

    let create_table = sqlx::query("CREATE TABLE IF NOT EXISTS users(id INTEGER
    PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&newdb).await.unwrap();
    println!("Create table users Result: {:?}", create_table);

    let get_tables = sqlx::query(
        "SELECT name
        FROM sqlite_schema
        WHERE type='table'
        AND name NOT LIKE 'sqlite_%'"
    )
    .fetch_all(&newdb)
    .await
    .unwrap();

    for (idx, val) in get_tables.iter().enumerate() {
        println!("[{}]: {:?}", idx, val.get::<String, &str>("name"));
    }
    // next we are going to insert data into table
    let insert_data = sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("new_name")
        .execute(&newdb)
        .await
        .unwrap();
    println!("The result of inserting data: {:?}", insert_data);
    // now extract the assigned new_name into a struct created above

    let user_result = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&newdb)
        .await
        .unwrap();
    println!("Extracting the existing user data...");

    for user in user_result{
        println!("{:?}", user);
    }
    // next planning to work on deleting
    let user_delete = sqlx::query("DELETE FROM users WHERE name=$1")
        .bind("new_name")
        .execute(&newdb)
        .await
        .unwrap();
    
    println!("Deleting user data...{:?}", user_delete);
    // using the migration scripts to create the accounts table

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("The crate dir is {:?}", crate_dir);
    // find where the migrations are
    let migrations = std::path::Path::new(&crate_dir)
        .join("./migrations");
    // apply the migrations to the database
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&newdb)
        .await;
    // check what happened to migration
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&newdb)
    .await
    .unwrap();
    // check the tables in the database
    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
    let result = sqlx::query("INSERT INTO accounts (name) VALUES (?)")
        .bind("achtung_baby")
        .execute(&newdb)
        .await
        .unwrap();

    println!("Query result: {:?}", result);

    let acc_results = sqlx::query_as::<_, Account>("SELECT id, name, active FROM accounts")
        .fetch_all(&newdb)
        .await
        .unwrap();
    // get the accounts printed
    for acc in acc_results {
        println!(
            "[{}] name: {}, active: {}",
            acc.id, &acc.name, acc.active
        );
    }
 
}
