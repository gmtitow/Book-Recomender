use crate::database::custom_postgresql::operations::CustomPostgres;

/**
 * dbms : 
 *          0 - postgresql
 */
pub struct DatabaseManager {
    dbms: i32,
    database_name : Option<String>,
    user_name : Option<String>,
    password : Option<String>,
    port : Option<String>
}

impl DatabaseManager {

    pub fn new(dbms: i32, user_name : String, password : String, database_name: String, port : Option<String>) 
                                -> DatabaseManager{
        DatabaseManager {
            dbms: dbms,
            database_name: Some(database_name),
            user_name: Some(user_name),
            password: Some(password),
            port: port
        }
    }

    pub fn into_postgresql(self) -> CustomPostgres {
        CustomPostgres::init(
            &self.user_name.expect("Не указано имя пользователя"), 
            &self.password.expect("Не указан пароль"), 
            &self.database_name.expect("Не указано имя базы данных")
        ).expect("Не удалось установить соединение")
    }
}