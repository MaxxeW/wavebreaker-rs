use crate::models::players::Player;
use crate::schema::rivalries;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Player, foreign_key = challenger_id))]
#[diesel(table_name = rivalries, check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(challenger_id, rival_id))]
pub struct Rivalry {
    pub challenger_id: i32,
    pub rival_id: i32,
    pub established_at: time::PrimitiveDateTime,
}

impl Rivalry {
    /// Find out whether or not the rivalry is mutual.
    pub async fn is_mutual(&self, conn: &mut AsyncPgConnection) -> bool {
        use crate::schema::rivalries::dsl::*;

        rivalries
            .filter(challenger_id.eq(self.rival_id))
            .filter(rival_id.eq(self.challenger_id))
            .get_result::<Self>(conn)
            .await
            .is_ok()
    }
}

#[derive(Insertable)]
#[diesel(table_name = rivalries)]
/// Represents a new rivalry between two players.
///
/// This struct is used to create a new rivalry by specifying the player ID and rival ID.
/// It provides a method to asynchronously create the rivalry in the database.
pub struct NewRivalry {
    pub challenger_id: i32,
    pub rival_id: i32,
}

impl NewRivalry {
    /// Creates a new `NewRivalry` instance.
    ///
    /// # Arguments
    ///
    /// * `player_id` - The ID of the player.
    /// * `rival_id` - The ID of the rival.
    ///
    /// # Returns
    ///
    /// A new `NewRivalry` instance.
    #[must_use]
    pub const fn new(challenger_id: i32, rival_id: i32) -> Self {
        Self {
            challenger_id,
            rival_id,
        }
    }

    /// Creates a new rivalry in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Returns
    ///
    /// A `QueryResult` containing the created `Rivalry` instance.
    ///
    /// # Errors
    /// This fails if:
    /// - The query fails
    pub async fn create(&self, conn: &mut AsyncPgConnection) -> QueryResult<Rivalry> {
        diesel::insert_into(rivalries::table)
            .values(self)
            .get_result(conn)
            .await
    }
}
