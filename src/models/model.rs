use diesel::*;
// use diesel::prelude::*;

pub trait Base {
    type Model;
    type Table: query_source::Table;
    type Connection;

    fn all(connection: &Self::Connection) -> QueryResult<Vec<Self::Model>> {
        Self::Table::table.load::<Self::Model>(&*connection)
    }

    fn get(id: i32, connection: &Self::Connection) -> QueryResult<Self::Model> {
        Self::Table::table.find(id).get_result::<Self::Model>(connection)
    }

    fn insert(model: Self::Model, connection: &Self::Connection) -> QueryResult<Self::Model> {
        diesel::insert_into(Self::Table::table)
            .values(&Insertable::<Self::Model>::from_model(model))
            .get_result(connection)
}

    fn update(id: i32, model: Self::Model, connection: &Self::Connection) -> QueryResult<Self::Model> {
        diesel::update(Self::Table::table.find(id))
            .set(&model)
            .get_result(connection)
    }

    fn delete(id: i32, connection: &Self::Connection) -> QueryResult<usize> {
        diesel::delete(Self::Table::table.find(id))
            .execute(connection)
    }
}
