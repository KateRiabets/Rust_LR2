#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use rocket::response::content::RawHtml;
use std::fs;

// Структура для відповіді з результатом
#[derive(serde::Serialize)]
struct CalcResult {
    result: Option<f64>,// Для результату обчислення
    error: Option<String>, // Для  помилки
}

// Відображення головної сторінки
#[get("/")]
fn index() -> Option<RawHtml<String>> {
    let html_content = fs::read_to_string("static/index.html").ok()?;
    Some(RawHtml(html_content))
}


#[get("/calculate?<expression>")]
fn calculate(expression: String) -> Json<CalcResult> {
    match eval_expression(&expression) { // Обробка виразу та повернення результату
        Ok(result) => {
            if result.is_infinite() || result.is_nan() { // Якщо нескінченність або nan
                Json(CalcResult {
                    result: None, // Результат не повертається
                    error: Some("Ділення на 0 неможливе".to_string()), // Помилка повертається
                })
            } else {
                Json(CalcResult {
                    result: Some(result), // Результат  повертається
                    error: None, // Помилка не повертається
                })
            }
        }
        Err(e) => Json(CalcResult {
            result: None, // Помилка - результат не повертається
            error: Some(e), // Повертається текст помилки
        }),
    }
}

// Розбір виразу та виконання операції
fn eval_expression(expression: &str) -> Result<f64, String> {
    if let Some((left, right)) = split_expression(expression, '+') {
        let left_value = parse_operand(left)?; // Лівий операнд
        let right_value = parse_operand(right)?; // Правий операнд
        return Ok(left_value + right_value);
    } else if let Some((left, right)) = split_expression(expression, '-') {
        let left_value = parse_operand(left)?; // Лівий операнд
        let right_value = parse_operand(right)?; // Правий операнд
        return Ok(left_value - right_value);
    } else if let Some((left, right)) = split_expression(expression, '*') {
        let left_value = parse_operand(left)?; // Лівий операнд
        let right_value = parse_operand(right)?; // Правий операнд
        return Ok(left_value * right_value);
    } else if let Some((left, right)) = split_expression(expression, '/') {
        let left_value = parse_operand(left)?; // Лівий операнд
        let right_value = parse_operand(right)?; // Правий операнд
        if right_value == 0.0 {
            return Err("Неможна ділити на 0".to_string());
        }
        return Ok(left_value / right_value);
    }

    Err("Неправильний вираз".to_string())
}

// Функція для розбору операндів
fn parse_operand(operand: &str) -> Result<f64, String> {
    operand.trim().parse::<f64>().map_err(|_| "Помилка при розборі числа".to_string())
}

fn split_expression(expression: &str, operator: char) -> Option<(&str, &str)> {
    let mut chars = expression.chars().enumerate(); // Прохід по символам рядка з індексацією
    let mut found_operator = false; // Змінна-індикатор знаходження знаку
    let mut index = 0; // Індекс знайденого оператора
    let mut in_number = false; // Для відстеження, чи поточний індекс у середині числа

    while let Some((i, c)) = chars.next() { // Проходід через всі символи виразу
        if c.is_digit(10) || c == '.' { // Якщо символ є частиною числа
            in_number = true;
        } else if c == '-' && (i == 0 || !in_number) {
            // Якщо це мінус і він на початку або не всередині числа (унарний мінус)
            in_number = false;
        } else if c == operator && i != 0 {
            // Оператор, якщо не на початку та не в середині числа
            found_operator = true; // Фіксація оператора
            index = i; // Запам'ятовування позиції
            break;
        } else {
            in_number = false;
        }
    }

    // Якщо оператор знайдено, розділення виразу на дві частини
    if found_operator {
        Some((&expression[..index], &expression[index + 1..]))
    } else {
        None
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, calculate])
}
