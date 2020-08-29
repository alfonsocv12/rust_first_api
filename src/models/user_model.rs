use super::schema::posts;

#[derive(Insertable)]
#[table_name="users"]

pub struct User<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
