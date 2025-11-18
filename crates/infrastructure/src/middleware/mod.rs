pub mod map_response;
pub mod mw_auth;
use axum::Json;
use core_crate::{AppResult, error::AppError};
use domain::user::{
    self, HasPrimary, User, request::{RequestCreateUser, RequestUpdateUser}
};
use modql::{
    SIden,
    field::{HasFields, HasSeaFields},
};
use sea_query::{
    Alias, Expr, ExprTrait, IntoIden, PostgresQueryBuilder, Query, Returning, TableRef,
};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, PgPool, postgres::PgRow};
use tracing::info;
// use tracing::info;
// fn table_ref()->TableRef{
//     TableRef::SchemaTable(SIden("user").into_iden(), SIden("tbl_user").into_iden())
// }
pub trait DMC {
    const SCHEMA: &'static str;
    const TABLE: &'static str;
    fn table_ref() -> TableRef {
        TableRef::SchemaTable(
            SIden(Self::SCHEMA).into_iden(),
            SIden(Self::TABLE).into_iden(),
        )
    }
}
pub struct TodoDMC;
impl DMC for TodoDMC {
    const SCHEMA: &'static str = "todo";
    const TABLE: &'static str = "tbl_todo_item";
}
pub async fn list<MC, O>(db: PgPool) -> AppResult<Json<Vec<O>>>
where
    O: for<'a> FromRow<'a, PgRow> + HasFields + Send + Unpin,
    MC: DMC,
{
    let list_data: Vec<O> = sqlx::query_as::<_, O>(
        format!("SELECT * FROM \"{}\".\"{}\"", MC::SCHEMA, MC::TABLE).as_str(),
    )
    .fetch_all(&db)
    .await?;
    Ok(Json(list_data))
}
pub async fn find_by_field<MC, O, T>(
    db: PgPool,
    field_name: &str,
    field_value: T,
) -> AppResult<Option<Vec<O>>>
where
    O: for<'a> FromRow<'a, PgRow> + HasFields + Send + Unpin +Clone,
    MC: DMC,
    T: sqlx::Type<sqlx::Postgres> + for<'a> sqlx::Encode<'a, sqlx::Postgres> + Send + Sync,
{
    let query = format!(
        "SELECT * FROM \"{}\".\"{}\" WHERE {} = $1",
        MC::SCHEMA,
        MC::TABLE,
        field_name
    );
    let result: Vec<O> = sqlx::query_as::<_, O>(&query)
        .bind(field_value)
        .fetch_all(&db)
        .await?;
    Ok(Some(result))
}
pub async fn update<MC, T>(db: PgPool, entity: T) -> AppResult<()>
where
    MC: DMC,
    T: HasSeaFields + for<'a> FromRow<'a, PgRow> + Send + Unpin + HasPrimary + Clone,
{
    let pk_field = T::PRIMARY_NAME;
    let cloned_entity = entity.clone();
    let pk_value = cloned_entity.primary_value();
    let sea_fields = entity.not_none_sea_fields();
    let mut query = Query::update();
    query.table(MC::table_ref());
    let sets = sea_fields.for_sea_update();
    for (col, val) in sets {
        info!("Set column: {:?}, value: {:?}", &col, &val);
        query.value(col, val);
    }
    query.and_where(Expr::col(Alias::new(pk_field)).eq(Expr::val(pk_value)));
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    info!("SQL: {}", sql);
    sqlx::query_with(&sql, values).execute(&db).await?;
    Ok(())
}
pub async fn delete<MC,T>(db: PgPool, user_id: T) -> AppResult<()> 
where
    MC: DMC,
    T:HasPrimary ,

{
    let primary_field= T::PRIMARY_NAME;
    let mut query=Query::delete();
    let primary_value=user_id.primary_value();
    query.from_table(MC::table_ref());
    query.and_where(Expr::col(Alias::new(primary_field)).eq(Expr::val(primary_value)));
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    info!("SQL: {}", sql);
    sqlx::query_with(&sql, values).execute(&db).await?;
    Ok(())
}
pub struct UserDMC;
impl DMC for UserDMC {
    const SCHEMA: &'static str = "user";
    const TABLE: &'static str = "tbl_user";
}
pub async fn create<MC, O>(db: PgPool, input: O) -> AppResult<O>
where
    MC: DMC,
    O: HasSeaFields + for<'a> FromRow<'a, PgRow> + Send + Unpin,
{
    //    let (columns, values)=user.
    //    println!("{}", user.username);

    // let values=input;
    let sea_fields = input.not_none_sea_fields();
    let (columns, values) = sea_fields.for_sea_insert();
    let mut query = Query::insert();

    query.into_table(MC::table_ref());
    query.columns(columns.clone()).values(values);
    query.returning(
        Returning::new().columns(
            O::field_names()
                .iter()
                .map(|name| Alias::new(*name))
                .collect::<Vec<_>>(),
        ),
    );
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    info!("SQL: {}", sql);
    let entity = sqlx::query_as_with::<_, _, _>(&sql, values)
        .fetch_one(&db)
        .await?;
    Ok(entity)
}
// ...existing code...
