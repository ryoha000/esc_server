use diesel::prelude::*;

use super::super::models;

/// Run query using Diesel to insert a new database row and return the result.
pub fn find_brand_by_id(
    _id: i32,
    conn: &PgConnection,
) -> Result<Option<models::Brand>, diesel::result::Error> {
    use crate::schema::brands::dsl::*;

    let brand = brands
        .filter(id.eq(_id))
        .first::<models::Brand>(conn)
        .optional()?;

    Ok(brand)
}

pub fn find_brands(
    conn: &PgConnection,
) -> Result<Option<Vec<models::Brand>>, diesel::result::Error> {
    use crate::schema::brands::dsl::*;

    let brand = brands
        .load::<models::Brand>(conn)
        .optional()?;

    Ok(brand)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_brand(
    new_brand: models::Brand,
    conn: &PgConnection,
) -> Result<models::Brand, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::brands::dsl::*;

    diesel::insert_into(brands).values(&new_brand).execute(conn)?;

    Ok(new_brand)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_brands(
    new_brands: Vec<models::Brand>,
    conn: &PgConnection,
) -> Result<Vec<models::Brand>, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::brands::dsl::*;

    diesel::insert_into(brands).values(&new_brands).execute(conn)?;

    Ok(new_brands)
}
