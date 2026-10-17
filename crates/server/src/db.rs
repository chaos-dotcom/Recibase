//! PostgreSQL persistence for the admin tracker (Diesel + r2d2).
//!
//! The schema lives in `crates/server/migrations` and is applied through an
//! embedded migration set, so the deployed binary owns its own migrations and
//! needs no `diesel` CLI at runtime.

use chrono::{DateTime, NaiveDate, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub mod schema {
    diesel::table! {
        users (id) {
            id -> Int4,
            subject -> Text,
            email -> Nullable<Text>,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        meal_entries (id) {
            id -> Int4,
            user_id -> Int4,
            meal_name -> Text,
            eaten_on -> Date,
            note -> Nullable<Text>,
            featured -> Bool,
            created_at -> Timestamptz,
        }
    }

    diesel::table! {
        sessions (id) {
            id -> Text,
            user_id -> Int4,
            csrf_token -> Text,
            created_at -> Timestamptz,
            expires_at -> Timestamptz,
        }
    }

    diesel::table! {
        auth_flows (state) {
            state -> Text,
            nonce -> Text,
            code_verifier -> Text,
            created_at -> Timestamptz,
            expires_at -> Timestamptz,
        }
    }

    diesel::table! {
        settings (key) {
            key -> Text,
            value -> Text,
            updated_at -> Timestamptz,
        }
    }

    diesel::joinable!(meal_entries -> users (user_id));
    diesel::joinable!(sessions -> users (user_id));
    diesel::allow_tables_to_appear_in_same_query!(users, meal_entries, sessions, auth_flows, settings);
}

use schema::{auth_flows, meal_entries, sessions, settings, users};

/// The `settings` key holding the adopted owner's OIDC subject.
pub const OWNER_SUBJECT: &str = "owner_subject";

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub subject: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = meal_entries)]
pub struct MealEntryRow {
    pub id: i32,
    pub user_id: i32,
    pub meal_name: String,
    pub eaten_on: NaiveDate,
    pub note: Option<String>,
    pub featured: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = meal_entries)]
pub struct NewMealEntry<'a> {
    pub user_id: i32,
    pub meal_name: &'a str,
    pub eaten_on: NaiveDate,
    pub note: Option<&'a str>,
    pub featured: bool,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: String,
    pub user_id: i32,
    pub csrf_token: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = auth_flows)]
pub struct AuthFlow {
    pub state: String,
    pub nonce: String,
    pub code_verifier: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// A cloneable handle around the connection pool, shared through `Arc` in the
/// request context.
#[derive(Clone)]
pub struct Db {
    pool: DbPool,
}

impl Db {
    /// Builds a pool and applies any pending migrations. Fails if the database
    /// is unreachable, so start-up can refuse to serve without its store.
    pub fn connect(database_url: &str) -> DbResult<Db> {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().max_size(8).build(manager)?;
        let db = Db { pool };
        db.run_migrations()?;
        Ok(db)
    }

    fn run_migrations(&self) -> DbResult<()> {
        let mut conn = self.conn()?;
        // A session-level advisory lock serialises concurrent starts (and the
        // parallel test harness): two processes must not create the same tables
        // at once. The lock is released when this connection closes.
        diesel::sql_query("SELECT pg_advisory_lock(1919115113)").execute(&mut conn)?;
        let outcome = conn
            .run_pending_migrations(MIGRATIONS)
            .map(|_applied| ())
            .map_err(|error| error.to_string());
        diesel::sql_query("SELECT pg_advisory_unlock(1919115113)").execute(&mut conn)?;
        outcome.map_err(|message| -> Box<dyn std::error::Error + Send + Sync> {
            message.into()
        })?;
        Ok(())
    }

    fn conn(&self) -> DbResult<DbConn> {
        Ok(self.pool.get()?)
    }

    /// `User` for an OIDC subject, created on first sight; a changed email is
    /// written back.
    pub fn upsert_user(&self, subject: &str, email: Option<&str>) -> DbResult<User> {
        let mut conn = self.conn()?;
        let existing: Option<User> = users::table
            .filter(users::subject.eq(subject))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?;
        match existing {
            Some(user) => {
                if user.email.as_deref() != email {
                    diesel::update(users::table.find(user.id))
                        .set(users::email.eq(email))
                        .execute(&mut conn)?;
                    let mut user = user;
                    user.email = email.map(str::to_string);
                    Ok(user)
                } else {
                    Ok(user)
                }
            }
            None => {
                let inserted: User = diesel::insert_into(users::table)
                    .values((users::subject.eq(subject), users::email.eq(email)))
                    .returning(User::as_returning())
                    .get_result(&mut conn)?;
                Ok(inserted)
            }
        }
    }

    pub fn user_by_subject(&self, subject: &str) -> DbResult<Option<User>> {
        let mut conn = self.conn()?;
        Ok(users::table
            .filter(users::subject.eq(subject))
            .select(User::as_select())
            .first(&mut conn)
            .optional()?)
    }

    /// Stores a new session and returns its opaque identifier.
    pub fn create_session(
        &self,
        user_id: i32,
        csrf_token: &str,
        expires_at: DateTime<Utc>,
    ) -> DbResult<String> {
        let mut conn = self.conn()?;
        let id = crate::tokens::random_hex(32);
        diesel::insert_into(sessions::table)
            .values((
                sessions::id.eq(&id),
                sessions::user_id.eq(user_id),
                sessions::csrf_token.eq(csrf_token),
                sessions::expires_at.eq(expires_at),
            ))
            .execute(&mut conn)?;
        Ok(id)
    }

    /// The session and its user, if the session exists and has not expired.
    pub fn session(&self, id: &str, now: DateTime<Utc>) -> DbResult<Option<(Session, User)>> {
        let mut conn = self.conn()?;
        let session: Option<Session> = sessions::table
            .find(id)
            .select(Session::as_select())
            .first(&mut conn)
            .optional()?;
        let Some(session) = session else {
            return Ok(None);
        };
        if session.expires_at <= now {
            diesel::delete(sessions::table.find(id)).execute(&mut conn)?;
            return Ok(None);
        }
        let user: User = users::table
            .find(session.user_id)
            .select(User::as_select())
            .first(&mut conn)?;
        Ok(Some((session, user)))
    }

    pub fn delete_session(&self, id: &str) -> DbResult<()> {
        let mut conn = self.conn()?;
        diesel::delete(sessions::table.find(id)).execute(&mut conn)?;
        Ok(())
    }

    /// Removes expired sessions and pending login flows; called opportunistically
    /// on login so the tables do not grow without bound.
    pub fn purge_expired(&self, now: DateTime<Utc>) -> DbResult<()> {
        let mut conn = self.conn()?;
        diesel::delete(sessions::table.filter(sessions::expires_at.lt(now))).execute(&mut conn)?;
        diesel::delete(auth_flows::table.filter(auth_flows::expires_at.lt(now)))
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn create_auth_flow(
        &self,
        state: &str,
        nonce: &str,
        code_verifier: &str,
        expires_at: DateTime<Utc>,
    ) -> DbResult<()> {
        let mut conn = self.conn()?;
        diesel::insert_into(auth_flows::table)
            .values((
                auth_flows::state.eq(state),
                auth_flows::nonce.eq(nonce),
                auth_flows::code_verifier.eq(code_verifier),
                auth_flows::expires_at.eq(expires_at),
            ))
            .execute(&mut conn)?;
        Ok(())
    }

    /// Consumes a pending login flow by state, deleting it so a `state` cannot
    /// be replayed.
    pub fn take_auth_flow(&self, state: &str) -> DbResult<Option<AuthFlow>> {
        let mut conn = self.conn()?;
        let flow: Option<AuthFlow> = diesel::delete(auth_flows::table.find(state))
            .returning(AuthFlow::as_returning())
            .get_result(&mut conn)
            .optional()?;
        Ok(flow)
    }

    /// A value from the `settings` table, if present.
    pub fn setting(&self, key: &str) -> DbResult<Option<String>> {
        let mut conn = self.conn()?;
        Ok(settings::table
            .find(key)
            .select(settings::value)
            .first(&mut conn)
            .optional()?)
    }

    /// Inserts or replaces a `settings` value.
    pub fn set_setting(&self, key: &str, value: &str) -> DbResult<()> {
        let mut conn = self.conn()?;
        diesel::insert_into(settings::table)
            .values((settings::key.eq(key), settings::value.eq(value)))
            .on_conflict(settings::key)
            .do_update()
            .set((
                settings::value.eq(value),
                settings::updated_at.eq(diesel::dsl::now),
            ))
            .execute(&mut conn)?;
        Ok(())
    }

    pub fn entries_for_user(&self, user_id: i32) -> DbResult<Vec<MealEntryRow>> {
        let mut conn = self.conn()?;
        Ok(meal_entries::table
            .filter(meal_entries::user_id.eq(user_id))
            .order(meal_entries::eaten_on.desc())
            .select(MealEntryRow::as_select())
            .load(&mut conn)?)
    }

    /// Every entry recorded for an OIDC subject, oldest first; the shape the
    /// usage aggregation consumes.
    pub fn entries_for_subject(&self, subject: &str) -> DbResult<Vec<MealEntryRow>> {
        let mut conn = self.conn()?;
        Ok(meal_entries::table
            .inner_join(users::table)
            .filter(users::subject.eq(subject))
            .order(meal_entries::eaten_on.asc())
            .select(MealEntryRow::as_select())
            .load(&mut conn)?)
    }

    pub fn add_entry(&self, new: &NewMealEntry<'_>) -> DbResult<MealEntryRow> {
        let mut conn = self.conn()?;
        Ok(diesel::insert_into(meal_entries::table)
            .values(new)
            .returning(MealEntryRow::as_returning())
            .get_result(&mut conn)?)
    }

    /// Deletes an entry the user owns. Returns whether a row was removed.
    pub fn delete_entry(&self, id: i32, user_id: i32) -> DbResult<bool> {
        let mut conn = self.conn()?;
        let removed = diesel::delete(
            meal_entries::table
                .find(id)
                .filter(meal_entries::user_id.eq(user_id)),
        )
        .execute(&mut conn)?;
        Ok(removed > 0)
    }
}
