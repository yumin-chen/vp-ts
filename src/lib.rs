use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use napi_derive::napi;
use std::sync::Mutex;

pub mod schema {
  diesel::table! {
    users (id) {
      id -> Integer,
      name -> Text,
      hair_color -> Nullable<Text>,
      email -> Nullable<Text>,
      banned -> Bool,
      organization_id -> Nullable<Integer>,
    }
  }

  diesel::table! {
    posts (id) {
      id -> Integer,
      title -> Text,
      body -> Text,
      published -> Bool,
      user_id -> Integer,
    }
  }

  diesel::table! {
    versions (id) {
      id -> Integer,
      num -> Text,
      crate_id -> Integer,
    }
  }

  diesel::table! {
    version_downloads (id) {
      id -> Integer,
      version_id -> Integer,
      downloads -> Integer,
      counted -> Integer,
      date -> Text,
    }
  }
}

#[napi(object)]
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
  pub id: i32,
  pub name: String,
  pub hair_color: Option<String>,
  pub email: Option<String>,
  pub banned: bool,
  pub organization_id: Option<i32>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUserRecord<'a> {
  pub name: &'a str,
  pub hair_color: Option<&'a str>,
  pub email: Option<&'a str>,
  pub banned: Option<bool>,
  pub organization_id: Option<i32>,
}

#[napi(object)]
pub struct NewUserParams {
  pub name: String,
  pub hair_color: Option<String>,
  pub email: Option<String>,
  pub banned: Option<bool>,
  pub organization_id: Option<i32>,
}

#[napi(object)]
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
  pub user_id: i32,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::posts)]
pub struct NewPostRecord<'a> {
  pub title: &'a str,
  pub body: &'a str,
  pub published: Option<bool>,
  pub user_id: i32,
}

#[napi(object)]
pub struct NewPostParams {
  pub title: String,
  pub body: String,
  pub published: Option<bool>,
  pub user_id: Option<i32>,
}

#[napi(object)]
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::versions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VersionModel {
  pub id: i32,
  pub num: String,
  pub crate_id: i32,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::versions)]
pub struct NewVersionRecord<'a> {
  pub num: &'a str,
  pub crate_id: i32,
}

#[napi(object)]
#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = schema::version_downloads)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Download {
  pub id: i32,
  pub version_id: i32,
  pub downloads: i32,
  pub counted: i32,
  pub date: String,
}

#[napi(object)]
pub struct NewDownloadParams {
  pub version_id: i32,
  pub downloads: i32,
  pub counted: i32,
  pub date: String,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::version_downloads)]
pub struct NewDownloadRecord<'a> {
  pub version_id: i32,
  pub downloads: i32,
  pub counted: i32,
  pub date: &'a str,
}

#[napi]
pub struct NativeConnection {
  conn: Mutex<SqliteConnection>,
}

#[napi]
impl NativeConnection {
  #[napi(constructor)]
  pub fn new(database_url: Option<String>) -> napi::Result<Self> {
    let url = database_url.unwrap_or_else(|| ":memory:".to_string());
    let conn = SqliteConnection::establish(&url)
      .map_err(|e| napi::Error::from_reason(format!("Error connecting to {}: {}", url, e)))?;
    Ok(Self {
      conn: Mutex::new(conn),
    })
  }

  #[napi]
  pub fn setup_tables(&self) -> napi::Result<()> {
    use diesel::connection::SimpleConnection;
    let mut conn = self.conn.lock().unwrap();
    let sql = "
      CREATE TABLE IF NOT EXISTS users (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        hair_color TEXT,
        email TEXT,
        banned BOOLEAN NOT NULL DEFAULT 0,
        organization_id INTEGER
      );
      CREATE TABLE IF NOT EXISTS posts (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        published BOOLEAN NOT NULL DEFAULT 0,
        user_id INTEGER NOT NULL DEFAULT 1
      );
      CREATE TABLE IF NOT EXISTS versions (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        num TEXT NOT NULL,
        crate_id INTEGER NOT NULL
      );
      CREATE TABLE IF NOT EXISTS version_downloads (
        id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
        version_id INTEGER NOT NULL,
        downloads INTEGER NOT NULL,
        counted INTEGER NOT NULL,
        date TEXT NOT NULL
      );
    ";
    conn.batch_execute(sql)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(())
  }

  #[napi]
  pub fn load_users(&self) -> napi::Result<Vec<User>> {
    use schema::users::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    users
      .select(User::as_select())
      .load(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn insert_users(&self, new_users: Vec<NewUserParams>) -> napi::Result<Vec<User>> {
    use schema::users::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    let records: Vec<NewUserRecord> = new_users
      .iter()
      .map(|u| NewUserRecord {
        name: &u.name,
        hair_color: u.hair_color.as_deref(),
        email: u.email.as_deref(),
        banned: u.banned,
        organization_id: u.organization_id,
      })
      .collect();

    diesel::insert_into(users)
      .values(&records)
      .returning(User::as_returning())
      .get_results(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn update_banned_by_email(&self, pattern: String, banned_status: bool) -> napi::Result<u32> {
    use schema::users::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    let count = diesel::update(users.filter(email.like(pattern)))
      .set(banned.eq(banned_status))
      .execute(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(count as u32)
  }

  #[napi]
  pub fn posts_belonging_to_user(&self, target_user_id: i32) -> napi::Result<Vec<Post>> {
    use schema::posts::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    posts
      .filter(user_id.eq(target_user_id))
      .select(Post::as_select())
      .load(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn create_post(&self, title_str: String, body_str: String, uid: Option<i32>) -> napi::Result<Post> {
    use schema::posts;
    let new_post = NewPostRecord {
      title: &title_str,
      body: &body_str,
      published: Some(false),
      user_id: uid.unwrap_or(1),
    };
    let mut conn = self.conn.lock().unwrap();
    diesel::insert_into(posts::table)
      .values(&new_post)
      .returning(Post::as_returning())
      .get_result(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn show_posts(&self, limit_val: Option<i64>) -> napi::Result<Vec<Post>> {
    use schema::posts::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    posts
      .filter(published.eq(true))
      .limit(limit_val.unwrap_or(5))
      .select(Post::as_select())
      .load(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn publish_post(&self, post_id: i32) -> napi::Result<Post> {
    use schema::posts::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    diesel::update(posts.find(post_id))
      .set(published.eq(true))
      .returning(Post::as_returning())
      .get_result(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn get_post(&self, post_id: i32) -> napi::Result<Option<Post>> {
    use schema::posts::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    posts
      .find(post_id)
      .select(Post::as_select())
      .first(&mut *conn)
      .optional()
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn delete_posts(&self, target: String) -> napi::Result<u32> {
    use schema::posts::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    let pattern = format!("%{}%", target);
    let count = diesel::delete(posts.filter(title.like(pattern)))
      .execute(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(count as u32)
  }

  #[napi]
  pub fn insert_version(&self, num_str: String, crate_id_val: i32) -> napi::Result<VersionModel> {
    use schema::versions;
    let mut conn = self.conn.lock().unwrap();
    let record = NewVersionRecord {
      num: &num_str,
      crate_id: crate_id_val,
    };
    diesel::insert_into(versions::table)
      .values(&record)
      .returning(VersionModel::as_returning())
      .get_result(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn query_downloads(&self, target_crate_id: i32, date_after: String, limit_val: i64) -> napi::Result<Vec<Download>> {
    use schema::versions::dsl as v_dsl;
    use schema::version_downloads::dsl as vd_dsl;

    let mut conn = self.conn.lock().unwrap();

    let version_ids: Vec<i32> = v_dsl::versions
      .filter(v_dsl::crate_id.eq(target_crate_id))
      .order(v_dsl::num.desc())
      .limit(limit_val)
      .select(v_dsl::id)
      .load(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    vd_dsl::version_downloads
      .filter(vd_dsl::date.gt(date_after))
      .filter(vd_dsl::version_id.eq_any(version_ids))
      .order(vd_dsl::date)
      .select(Download::as_select())
      .load(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn insert_downloads(&self, downloads_input: Vec<NewDownloadParams>) -> napi::Result<Vec<Download>> {
    use schema::version_downloads::dsl::*;
    let mut conn = self.conn.lock().unwrap();
    let records: Vec<NewDownloadRecord> = downloads_input
      .iter()
      .map(|d| NewDownloadRecord {
        version_id: d.version_id,
        downloads: d.downloads,
        counted: d.counted,
        date: &d.date,
      })
      .collect();

    diesel::insert_into(version_downloads)
      .values(&records)
      .returning(Download::as_returning())
      .get_results(&mut *conn)
      .map_err(|e| napi::Error::from_reason(e.to_string()))
  }

  #[napi]
  pub fn execute_raw(&self, sql: String) -> napi::Result<u32> {
    use diesel::connection::SimpleConnection;
    let mut conn = self.conn.lock().unwrap();
    conn.batch_execute(&sql)
      .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(0)
  }
}

#[napi]
pub fn establish_connection(database_url: Option<String>) -> napi::Result<NativeConnection> {
  NativeConnection::new(database_url)
}
