use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::review::Review;
use crate::models::user::User;
use crate::schema::schema::review_user_liked;

#[derive(
    Identifiable, Selectable, Serialize, Deserialize, Queryable, Associations, Debug, Clone,
)]
#[diesel(belongs_to(Review, foreign_key = id_review))]
#[diesel(belongs_to(User, foreign_key = id_user))]
#[diesel(table_name = review_user_liked)]
#[diesel(primary_key(id_review, id_user))]
pub struct ReviewUserLiked {
    pub id_review: u32,
    pub id_user: u32,
}
