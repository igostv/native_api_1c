use std::sync::Arc;

use native_api_1c::{native_api_1c_core::ffi::connection::Connection, native_api_1c_macro::AddIn};

#[derive(AddIn)]
pub struct MyAddIn {
    // соедиенение с 1С для вызова внешних событий
    #[add_in_con]
    connection: Arc<Option<&'static Connection>>, // Arc для возможности многопоточности

    // свойство, доступное для чтения и записи
    #[add_in_prop(name = "MyProp", name_ru = "МоеСвойство", readable, writable)]
    pub some_prop: i32,

    // свойство, доступное только для чтения
    #[add_in_prop(name = "ProtectedProp", name_ru = "ЗащищенноеСвойство", readable)]
    pub protected_prop: i32,

    // функция, принимающая один или два аргумента и возвращающая результат
    // в 1С можно вызвать как:
    //  ОбъектКомпоненты.МояФункция(10, 15); // 2й аргумент = 15
    //  ОбъектКомпоненты.МояФункция(10);     // 2й аргумент = 12 (значение по умолчанию)
    // Если функция возвращает ошибку, но не паника, то в 1С будет вызвано исключение
    #[add_in_func(name = "MyFunction", name_ru = "МояФункция")]
    #[arg(Int)]
    #[arg(Int, default = 12)]
    #[returns(Int, result)]
    pub my_function: fn(&Self, i32, i64) -> Result<i32, ()>,

    // Процедура, ничего не получающая, ничего не возвращающая
    #[add_in_func(name = "MyProcedure", name_ru = "МояПроцедура")]
    pub my_procedure: fn(&mut Self),

    private_field: i32,
}

impl MyAddIn {
    pub fn new() -> Self {
        Self {
            connection: Arc::new(None),
            some_prop: 0,
            protected_prop: 50,
            my_function: Self::my_function,
            my_procedure: Self::my_procedure,
            private_field: 100,
        }
    }

    fn my_function(&self, arg: i32, arg_maybe_default: i64) -> Result<i32, ()> {
        Ok(self.protected_prop
            + self.some_prop
            + arg
            + self.private_field
            + arg_maybe_default as i32)
    }

    fn my_procedure(&mut self) {
        self.protected_prop += 1;
    }
}
