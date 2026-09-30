use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use napi_derive::napi;

// Diesel table macros
diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        hair_color -> Nullable<Text>,
        banned -> Bool,
        organization_id -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    downloads (id) {
        id -> Integer,
        version_id -> Integer,
        #[sql_name = "downloads"]
        downloads_count -> Integer,
        counted -> Integer,
        date -> Text,
    }
}

// Diesel model structs
#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub banned: bool,
    pub organization_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub hair_color: Option<&'a str>,
    pub banned: bool,
    pub organization_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(User))]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub published: bool,
}

#[derive(Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = downloads)]
pub struct Download {
    pub id: i32,
    pub version_id: i32,
    pub downloads_count: i32,
    pub counted: i32,
    pub date: String,
}

#[derive(Insertable)]
#[diesel(table_name = downloads)]
pub struct NewDownload<'a> {
    pub version_id: i32,
    #[diesel(column_name = downloads_count)]
    pub downloads_count: i32,
    pub counted: i32,
    pub date: &'a str,
}

// NAPI exported JS interface objects
#[napi(object)]
#[derive(Clone)]
pub struct JsUser {
    pub id: i32,
    pub name: String,
    pub hair_color: Option<String>,
    pub banned: bool,
    pub organization_id: i32,
}

#[napi(object)]
pub struct JsNewUser {
    pub name: String,
    pub hair_color: Option<String>,
    pub banned: Option<bool>,
    pub organization_id: Option<i32>,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsPost {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[napi(object)]
pub struct JsNewPost {
    pub user_id: Option<i32>,
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}

#[napi(object)]
#[derive(Clone)]
pub struct JsDownload {
    pub id: i32,
    pub version_id: i32,
    pub downloads: i32,
    pub counted: i32,
    pub date: String,
}

#[napi(object)]
pub struct JsNewDownload {
    pub version_id: i32,
    pub downloads: i32,
    pub counted: i32,
    pub date: String,
}

// NAPI Diesel Orm Engine Class
#[napi]
pub struct DieselOrm {
    conn: SqliteConnection,
}

#[napi]
impl DieselOrm {
    #[napi(constructor)]
    pub fn new(database_url: String) -> napi::Result<Self> {
        let mut conn = SqliteConnection::establish(&database_url)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        // Create schema tables if not existing
        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                hair_color TEXT,
                banned BOOLEAN NOT NULL DEFAULT 0,
                organization_id INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(&mut conn)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS posts (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL DEFAULT 0,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                published BOOLEAN NOT NULL DEFAULT 0
            )",
        )
        .execute(&mut conn)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        diesel::sql_query(
            "CREATE TABLE IF NOT EXISTS downloads (
                id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                version_id INTEGER NOT NULL,
                downloads INTEGER NOT NULL,
                counted INTEGER NOT NULL,
                date TEXT NOT NULL
            )",
        )
        .execute(&mut conn)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(DieselOrm { conn })
    }

    /// Loading all users from database: users::table.load(&mut connection)
    #[napi]
    pub fn load_all_users(&mut self) -> napi::Result<Vec<JsUser>> {
        use crate::users::dsl::*;
        let results = users
            .select(User::as_select())
            .load(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(|u| JsUser {
                id: u.id,
                name: u.name,
                hair_color: u.hair_color,
                banned: u.banned,
                organization_id: u.organization_id,
            })
            .collect())
    }

    /// Inserting data with RETURNING results: insert_into(users).values(&new_users).get_results(...)
    #[napi]
    pub fn insert_users(&mut self, new_users: Vec<JsNewUser>) -> napi::Result<Vec<JsUser>> {
        use crate::users::dsl::*;

        let records: Vec<NewUser> = new_users
            .iter()
            .map(|u| NewUser {
                name: &u.name,
                hair_color: u.hair_color.as_deref(),
                banned: u.banned.unwrap_or(false),
                organization_id: u.organization_id.unwrap_or(0),
            })
            .collect();

        let inserted: Vec<User> = diesel::insert_into(users)
            .values(&records)
            .returning(User::as_returning())
            .get_results(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(inserted
            .into_iter()
            .map(|u| JsUser {
                id: u.id,
                name: u.name,
                hair_color: u.hair_color,
                banned: u.banned,
                organization_id: u.organization_id,
            })
            .collect())
    }

    /// Loading all posts for a user: Post::belonging_to(user).load(&mut connection)
    #[napi]
    pub fn load_posts_for_user(&mut self, user_id_param: i32) -> napi::Result<Vec<JsPost>> {
        let user = User {
            id: user_id_param,
            name: String::new(),
            hair_color: None,
            banned: false,
            organization_id: 0,
        };

        let user_posts: Vec<Post> = Post::belonging_to(&user)
            .select(Post::as_select())
            .load(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(user_posts
            .into_iter()
            .map(|p| JsPost {
                id: p.id,
                user_id: p.user_id,
                title: p.title,
                body: p.body,
                published: p.published,
            })
            .collect())
    }

    /// Creating a new post: insert_into(posts).values(...).returning(...).get_result(...)
    #[napi]
    pub fn create_post(&mut self, post: JsNewPost) -> napi::Result<JsPost> {
        use crate::posts::dsl::*;

        let new_post = NewPost {
            user_id: post.user_id.unwrap_or(0),
            title: &post.title,
            body: &post.body,
            published: post.published.unwrap_or(false),
        };

        let inserted: Post = diesel::insert_into(posts)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(JsPost {
            id: inserted.id,
            user_id: inserted.user_id,
            title: inserted.title,
            body: inserted.body,
            published: inserted.published,
        })
    }

    /// Get single post with .optional() -> Option<Post>
    #[napi]
    pub fn get_post(&mut self, post_id: i32) -> napi::Result<Option<JsPost>> {
        use crate::posts::dsl::*;

        let post_opt: Option<Post> = posts
            .find(post_id)
            .select(Post::as_select())
            .first(&mut self.conn)
            .optional()
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(post_opt.map(|p| JsPost {
            id: p.id,
            user_id: p.user_id,
            title: p.title,
            body: p.body,
            published: p.published,
        }))
    }

    /// Publish post: update(posts.find(id)).set(published.eq(true))
    #[napi]
    pub fn publish_post(&mut self, post_id: i32) -> napi::Result<JsPost> {
        use crate::posts::dsl::*;

        let updated: Post = diesel::update(posts.find(post_id))
            .set(published.eq(true))
            .returning(Post::as_returning())
            .get_result(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(JsPost {
            id: updated.id,
            user_id: updated.user_id,
            title: updated.title,
            body: updated.body,
            published: updated.published,
        })
    }

    /// Show published posts with limit
    #[napi]
    pub fn show_published_posts(&mut self, limit_val: Option<i32>) -> napi::Result<Vec<JsPost>> {
        use crate::posts::dsl::*;

        let limit_num = limit_val.unwrap_or(5) as i64;

        let results: Vec<Post> = posts
            .filter(published.eq(true))
            .limit(limit_num)
            .select(Post::as_select())
            .load(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(|p| JsPost {
                id: p.id,
                user_id: p.user_id,
                title: p.title,
                body: p.body,
                published: p.published,
            })
            .collect())
    }

    /// Batch update: update(users.filter(name.like(...))).set(banned.eq(true))
    #[napi]
    pub fn batch_ban_users(&mut self, pattern: String) -> napi::Result<i32> {
        use crate::users::dsl::*;

        let count = diesel::update(users.filter(name.like(pattern)))
            .set(banned.eq(true))
            .execute(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(count as i32)
    }

    /// Delete posts matching title pattern: delete(posts.filter(title.like(pattern))).execute(...)
    #[napi]
    pub fn delete_posts(&mut self, pattern: String) -> napi::Result<i32> {
        use crate::posts::dsl::*;

        let count = diesel::delete(posts.filter(title.like(pattern)))
            .execute(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(count as i32)
    }

    /// Insert downloads
    #[napi]
    pub fn insert_downloads(&mut self, new_downloads: Vec<JsNewDownload>) -> napi::Result<Vec<JsDownload>> {
        use crate::downloads::dsl::*;

        let records: Vec<NewDownload> = new_downloads
            .iter()
            .map(|d| NewDownload {
                version_id: d.version_id,
                downloads_count: d.downloads,
                counted: d.counted,
                date: &d.date,
            })
            .collect();

        let inserted: Vec<Download> = diesel::insert_into(downloads)
            .values(&records)
            .returning(Download::as_returning())
            .get_results(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(inserted
            .into_iter()
            .map(|d| JsDownload {
                id: d.id,
                version_id: d.version_id,
                downloads: d.downloads_count,
                counted: d.counted,
                date: d.date,
            })
            .collect())
    }

    /// Complex query: downloads.filter(date.gt(...)).filter(version_id.eq_any(...)).order(date).limit(...)
    #[napi]
    pub fn get_recent_downloads_for_versions(
        &mut self,
        version_ids: Vec<i32>,
        min_date: String,
        limit_val: Option<i32>,
    ) -> napi::Result<Vec<JsDownload>> {
        use crate::downloads::dsl::*;

        let limit_num = limit_val.unwrap_or(5) as i64;

        let results: Vec<Download> = downloads
            .filter(date.gt(min_date))
            .filter(version_id.eq_any(version_ids))
            .order(date.asc())
            .limit(limit_num)
            .select(Download::as_select())
            .load(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(results
            .into_iter()
            .map(|d| JsDownload {
                id: d.id,
                version_id: d.version_id,
                downloads: d.downloads_count,
                counted: d.counted,
                date: d.date,
            })
            .collect())
    }

    /// Raw SQL execution: sql_query(...)
    #[napi]
    pub fn execute_raw_sql(&mut self, sql: String) -> napi::Result<i32> {
        let count = diesel::sql_query(sql)
            .execute(&mut self.conn)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

        Ok(count as i32)
    }
}
