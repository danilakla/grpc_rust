README.md
markdown
Копировать код
# GRPC Calculator with Rust

## Описание
Данное приложение реализует клиент-серверную архитектуру с использованием gRPC. Оно включает методы для выполнения базовых математических операций: сложение (`Add`), умножение (`Mul`), вычитание (`Sub`), деление (`Div`) и возведение в квадрат (`Pow2`). Вы сможете запустить сервер, а также клиент, который отправляет запросы серверу для выполнения указанных операций.

---

## Требования
1. **Rust**: Для разработки приложения.
2. **Protobuf Compiler** (`protoc`): Для генерации gRPC кода из `.proto` файла.
3. **Tonic**: Rust-библиотека для реализации gRPC.
4. **Tokio**: Асинхронный рантайм для работы с gRPC.

---

## Шаги по установке и настройке

### 1. Установить Rust
Для установки Rust воспользуйтесь [официальным установщиком](https://rustup.rs/):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Проверьте, что Rust установлен:

rustc --version
cargo --version
## 2. Установить Protobuf Compiler (protoc)

gRPC использует Protobuf для сериализации данных. Скачайте и установите protoc:

### Ubuntu/Debian:
sudo apt update
sudo apt install -y protobuf-compiler
protoc --version  # Проверить версию

Windows:
Скачайте precompiled binary отсюда (https://github.com/protocolbuffers/protobuf/releases). Добавьте protoc в PATH.

3. Создать проект Rust
Создайте новый проект:

cargo new grpc_calculator
cd grpc_calculator
4. Установить зависимости
Добавьте следующие зависимости в Cargo.toml:

[dependencies]
tonic = "0.9"
prost = "0.11"
tokio = { version = "1", features = ["full"] }

5. Подготовить Protobuf файл
Создайте папку proto в корне проекта:

mkdir proto
Создайте файл proto/calculator.proto:

syntax = "proto3";
package greet;

service Greeter {
  rpc Add (Parm2Request) returns (Parm2Result);
  rpc Mul (Parm2Request) returns (Parm2Result);
  rpc Sub (Parm2Request) returns (Parm2Result);
  rpc Div (Parm2Request) returns (Parm2Result);
  rpc Pow2 (Parm1Request) returns (Parm1Result);
}

message Parm2Request {
  int32 x = 1;
  int32 y = 2;
}

message Parm2Result {
  int32 x = 1;
  int32 y = 2;
  int32 z = 3;
}

message Parm1Request {
  int32 x = 1;
}

message Parm1Result {
  int32 x = 1;
  int32 z = 2;
}

6. Генерация gRPC кода
Добавьте build.rs в корневую папку проекта:

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/calculator.proto")?;
    Ok(())
}
Обновите Cargo.toml для сборки Protobuf:

cargo build
Сгенерированные файлы появятся в директории target/debug/build/grpc_calculator/.

7. Написание сервера
Создайте файл src/server.rs:

1. Импортирование зависимостей
rust
Копировать код
use tonic::{transport::Server, Request, Response, Status};
use greet::greeter_server::{Greeter, GreeterServer};
use greet::{Parm1Request, Parm1Result, Parm2Request, Parm2Result};
tonic: Это библиотека для работы с gRPC в Rust. Она обеспечивает все необходимое для создания сервера и клиента gRPC.
Request, Response, Status: Эти структуры используются для работы с запросами, ответами и статусами ошибок.
greeter_server::{Greeter, GreeterServer}: Эти типы автоматически генерируются из .proto файла. GreeterServer — это сервер, который обрабатывает запросы, а Greeter — это трейты с методами, которые нужно реализовать для обработки запросов.
Parm1Request, Parm1Result, Parm2Request, Parm2Result: Структуры, которые представляют запросы и ответы для операций, например, для сложения, вычитания и так далее.
2. Подключение Protobuf
rust
Копировать код
pub mod greet {
    tonic::include_proto!("greet");
}
Здесь мы подключаем Protobuf файл greet.proto, который должен быть сгенерирован с помощью tonic::include_proto!. Это автоматически подключает типы, такие как Greeter, Parm1Request, Parm1Result, и другие, на основе .proto файла.

3. Определение структуры сервера
rust
Копировать код
#[derive(Debug, Default)]
pub struct MyGreeter;
Здесь мы создаем структуру MyGreeter, которая будет обрабатывать все gRPC запросы. Структура MyGreeter реализует трейты для gRPC сервисов. Использование #[derive(Debug, Default)] автоматически генерирует код для отладки и для создания экземпляра структуры по умолчанию.

4. Реализация трейта Greeter
rust
Копировать код
#[tonic::async_trait]
impl Greeter for MyGreeter {
В этом блоке мы реализуем методы сервиса для каждой операции: сложение (add), умножение (mul), вычитание (sub), деление (div) и возведение в квадрат (pow2). Каждый метод асинхронный и принимает запрос типа Request<T>, где T — это структура данных, определенная в Protobuf.

Пример для метода add:
rust
Копировать код
async fn add(&self, request: Request<Parm2Request>) -> Result<Response<Parm2Result>, Status> {
    let req = request.into_inner();
    Ok(Response::new(Parm2Result {
        x: req.x,
        y: req.y,
        z: req.x + req.y,
    }))
}
Метод add получает запрос типа Parm2Request, который содержит два числа (например, x и y).
Мы используем request.into_inner(), чтобы извлечь данные из запроса.
Затем выполняем сложение чисел и создаем объект ответа Parm2Result с результатом (z).
В конце мы возвращаем ответ с результатом с помощью Response::new.
Другие методы (mul, sub, div, pow2) выполняют аналогичные операции для соответствующих математических операций.

Особенности:

Для метода div добавлена проверка на деление на ноль. Если y равен 0, возвращается ошибка Status::invalid_argument("Division by zero").
Метод pow2 возводит число в квадрат.
5. Запуск сервера
rust
Копировать код
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
#[tokio::main]: Атрибут для асинхронного основного блока программы с использованием Tokio. Это позволяет использовать асинхронные операции, такие как запуск gRPC сервера.
let addr = "[::1]:50051".parse()?;: Здесь мы задаем адрес, на котором будет слушать сервер. [::1] — это адрес для localhost (IPv6), и порт 50051.
Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;: Это создает и запускает сервер gRPC:
GreeterServer::new(greeter) — это создание сервера gRPC, который использует структуру greeter (в нашем случае, это экземпляр MyGreeter).
serve(addr) запускает сервер и начинает принимать входящие запросы на заданном адресе.


8. Написание клиента
Создайте файл src/client.rs:


Этот код представляет собой клиента для взаимодействия с сервером gRPC, который выполняет математические операции (сложение, вычитание, умножение, деление и возведение в квадрат). Клиент подключается к серверу, отправляет запросы и выводит результаты на экран. Давайте рассмотрим код более детально.

1. Импортирование зависимостей
rust
Копировать код
use tonic::transport::Channel;
use greet::greeter_client::GreeterClient;
use greet::{Parm1Request, Parm2Request};
tonic::transport::Channel: Это модуль для создания gRPC канала, через который клиент будет общаться с сервером.
greet::greeter_client::GreeterClient: Это клиент, сгенерированный на основе вашего .proto файла, который позволяет клиенту отправлять запросы на сервер.
greet::{Parm1Request, Parm2Request}: Структуры, сгенерированные из .proto файла, которые представляют данные запросов, которые отправляются на сервер.
2. Подключение Protobuf
rust
Копировать код
pub mod greet {
    tonic::include_proto!("greet");
}
Этот блок подключает сгенерированные типы из вашего .proto файла, что позволяет использовать типы данных, такие как Parm1Request, Parm1Result, и другие, для создания и обработки запросов и ответов.

3. Основная асинхронная функция
rust
Копировать код
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
#[tokio::main]: Атрибут, который позволяет использовать асинхронные функции с Tokio (библиотека для асинхронного программирования).
GreeterClient::connect("http://[::1]:50051").await?: Устанавливаем соединение с сервером, который работает на локальном хосте (IPv6 ::1) на порту 50051.
4. Цикл взаимодействия с пользователем
rust
Копировать код
loop {
    println!("Choose an operation: Add, Sub, Mul, Div, Pow2");
    let mut operation = String::new();
    std::io::stdin().read_line(&mut operation)?;
    let operation = operation.trim().to_lowercase();
loop {}: Цикл, который продолжает выполняться, пока не будет завершен пользователем.
Программа выводит сообщение, чтобы пользователь выбрал операцию, и ждет ввода.
Вводимая строка преобразуется в нижний регистр для удобства.
5. Обработка операций
rust
Копировать код
match operation.as_str() {
    "add" | "sub" | "mul" | "div" => {
        let (x, y) = get_two_numbers();
        let request = tonic::Request::new(Parm2Request { x, y });

        let response = match operation.as_str() {
            "add" => client.add(request).await?.into_inner(),
            "sub" => client.sub(request).await?.into_inner(),
            "mul" => client.mul(request).await?.into_inner(),
            "div" => client.div(request).await?.into_inner(),
            _ => unreachable!(),
        };
        println!("Result: x = {}, y = {}, z = {}", response.x, response.y, response.z);
    }
    "pow2" => {
        let x = get_one_number();
        let request = tonic::Request::new(Parm1Request { x });

        let response = client.pow2(request).await?.into_inner();
        println!("Result: x = {}, z = {}", response.x, response.z);
    }
    _ => {
        println!("Invalid operation. Try again.");
    }
}
Здесь осуществляется обработка введенной операции:

Если операция относится к бинарным операциям (add, sub, mul, div), запрашиваются два числа (вызов функции get_two_numbers()), создается запрос Parm2Request, и отправляется на сервер. В зависимости от операции вызывается соответствующий метод на сервере (add, sub, mul, div).
Для операции pow2 запрашивается одно число (вызов функции get_one_number()), и создается запрос Parm1Request, который отправляется на сервер для выполнения операции возведения в квадрат.
После выполнения операции выводится результат.
6. Функции для получения чисел
rust
Копировать код
fn get_two_numbers() -> (i32, i32) {
    println!("Enter two integers (x and y):");

    let x = read_number("x");
    let y = read_number("y");

    (x, y)
}

fn get_one_number() -> i32 {
    println!("Enter an integer (x):");
    read_number("x")
}

fn read_number(prompt: &str) -> i32 {
    loop {
        println!("{}: ", prompt);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse() {
            return num;
        }
        println!("Invalid input. Please enter a valid integer.");
    }
}
get_two_numbers и get_one_number — это функции для получения чисел от пользователя. Первая запрашивает два числа, а вторая — одно.
read_number — универсальная функция, которая спрашивает пользователя, вводит число, проверяет, что это целое число, и возвращает его. Если введено нечисловое значение, функция повторяет запрос.
7. Завершение работы
rust
Копировать код
Ok(())
Если все прошло успешно, функция main возвращает Ok(()), что сигнализирует об успешном завершении программы.



9. Запуск
Запустите сервер:
cargo run --bin server
Запустите клиент:
cargo run --bin client
Как это работает
proto/calculator.proto: Описание gRPC сервиса и сообщений.
server.rs: Реализация сервера, обрабатывающего запросы.
client.rs: Клиент для отправки запросов серверу.
build.rs: Генерация gRPC-кода из .proto.
