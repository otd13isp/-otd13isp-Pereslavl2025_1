// Actix Web — веб-фреймворк для Rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// обеспечивает поиск в строках совпадений с регулярным выражением
use regex::Regex;
// Serde предоставляет реализации десериализации для многих типов примитивов и стандартных библиотек Rust
use serde::Deserialize;
// модуль содержит базовые методы для управления содержимым локальной файловой системы
use std::fs;
// для создания нового потока необходимо вызвать функцию thread::spawn
use std::thread;
//
use duckdb::{params, Connection, Result};

// static MY_PATH: &str = "/home/user1/Документы/Sergey/Rust/bookrust2/public/searchdb.duckdb";
static MY_PATH: &str = "/home/user1/Документы/Sergey/rust/pereslavl2025/1/public/searchdb.duckdb";

/* ****************************************************** */
// запуск сервера
#[actix_web::main]
// std::io::Result<()> - специализированный тип результата для операций ввода-вывода
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(search))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

/* ****************************************************** */
// обработка запросов GET
#[get("/")]
async fn hello() -> impl Responder {
    let contents = fs::read_to_string("public/html/index.html")
        .expect("Should have been able to read the file");
    HttpResponse::Ok().body(contents)
}

/* ****************************************************** */
fn search_in_table(tablename: String, query_l: String) -> Result<Vec<String>> {
    // создание вектора для вывода строк, содержащих образец
    let mut result = Vec::new();
    let sql_query = format!(
        "{}{}{}",
        "SELECT textdisplay FROM ", tablename, " WHERE regexp_matches(searchtext, ?);"
    );
    println!("********* = {:?}", sql_query);
    let path = MY_PATH;
    let conn = Connection::open(&path)?;
    println!("conn.is_autocommit() = {}", conn.is_autocommit());
    
    // query table by rows
    let mut stmt = conn.prepare(&sql_query)?;    
    let row_iter = stmt.query_map(params![&query_l], |row| {
        Ok(
            row.get(0)?
        )
    })?;
    for one_row in row_iter {
        let r: String = one_row.unwrap();
        // println!("xxx: {:?}", r);
        result.push(r.clone());
    }    
    // для отладки использовать команду println!("Найдено в файле: {:?}", result);
    println!("result={:?}", result);
    Ok(result)
}

/* ****************************************************** */
// использование структуры для считывания параметров
#[derive(Deserialize)]
struct FormData {
    search: String,
}
// обработка запросов POST
#[post("/search")]
async fn search(form: web::Form<FormData>) -> impl Responder {
    let search = format!("{}", form.search);
    let search = search.to_lowercase();
    // предполагаем, что для строки поиска была проведена лемматизация
    // удаляем пробелы справа и слева строки
    let search = search.trim();
    // заменяем несколько пробелов на регулярное выражение
    let re = Regex::new(r"\s+").unwrap();
    let search = re.replace_all(&search, "( | .* )");
    // добавляем регулярные выражения в начало и конец строки поиска
    let search = format!("{}{}{}", "(^|.* )", search, "( .*$|$)");
    println!("search={}", search);
    // определяем вектор table_names, состоящий из строк
    let table_names = vec!["table1".to_string(), "table2".to_string()];
    let mut my_threads = vec![];
    for table_name in table_names {
        let search_aux = search.clone();
        // создаем потоки, передавая в них имена файлов из вектора file_names
        my_threads.push(thread::spawn(move || {
            let a = search_in_table(table_name, search_aux);
            let my_vec = match a {
                Ok(v) => v,
                Err(error) => panic!("Problem: {error:?}"),
            };            
        my_vec    
        }));
    }
    let results = my_threads.into_iter().map(|handle| handle.join().unwrap());
    // формирование результата
    let mut my_out = String::new();
    for v in results {
        for mut s in v {
            // println!("Найденная строка: {}", s);
            s.push_str("<br>");
            my_out.push_str(&s);
        }
    }
    HttpResponse::Ok().body(my_out)
}
