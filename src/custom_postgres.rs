extern crate postgres;

use postgres::{Connection, TlsMode, Error};

pub struct Book {
    pub name: String,
}

pub fn create_connection(user: &str, password: &str, database: &str)->Result<Connection,Error> {
    let url = format!("postgresql://{}:{}@localhost/{}",user,password,database);
    Connection::connect(url.to_string(), TlsMode::None)
}

pub fn insert_book(book: &Book, conn: &Connection)->Result<u64,Error>{
    conn.execute("insert into books (name) values($1)", &[&book.name])
}