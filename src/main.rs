use std::ffi::CString;
use std::env;
use std::io;
use tgwsproxy::{StartProxy, StopProxy};

fn main() {
    let mut host = String::from("127.0.0.1");
    let mut port = 1443;
    let mut secret = String::from("00000000000000000000000000000000");

    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--host" => {
                if i + 1 < args.len() {
                    host = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Ошибка: после --host ожидается значение.");
                    return;
                }
            }
            "--port" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<i32>() {
                        Ok(p) => port = p,
                        Err(_) => {
                            eprintln!("Неверный формат порта '{}', используется значение по умолчанию: {}", args[i + 1], 1443);
                        }
                    }
                    i += 1;
                } else {
                    eprintln!("Ошибка: после --port ожидается значение.");
                    return;
                }
            }
            "--secret" => {
                if i + 1 < args.len() {
                    secret = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Ошибка: после --secret ожидается значение.");
                    return;
                }
            }
            "--help" | "-h" => {
                println!("Использование: tgwsproxy.exe [OPTIONS]");
                println!("Options:");
                println!("  --host <HOST>      IP-адрес хоста (по умолчанию: 127.0.0.1)");
                println!("  --port <PORT>      Порт (по умолчанию: 1443)");
                println!("  --secret <SECRET>  Секрет, 32 символа (по умолчанию: 00000000000000000000000000000000)");
                return;
            }
            _ => {
                eprintln!("Неизвестный аргумент: '{}'. Используйте --help для справки.", args[i]);
                return;
            }
        }
        i += 1;
    }

    if secret.len() != 32 {
        eprintln!("Предупреждение: длина секрета должна быть ровно 32 шестнадцатеричных символа. Текущая длина: {}", secret.len());
    }

    println!("Запуск прокси на {}:{} ...", host, port);

    let c_host = CString::new(host.as_str()).unwrap();
    let c_dc_ips = CString::new("").unwrap();
    let c_secret = CString::new(secret.as_str()).unwrap();

    let result = unsafe {
        StartProxy(
            c_host.as_ptr(),
            port,
            c_dc_ips.as_ptr(),
            c_secret.as_ptr(),
            1,
        )
    };

    if result == 0 {
        println!("\nПрокси успешно запущен!");
        println!("Настройте Telegram на использование MTProto прокси:");
        let tg_link = format!("tg://proxy?server={}&port={}&secret={}", host, port, secret);
        let https_link = format!("https://t.me/proxy?server={}&port={}&secret={}", host, port, secret);
        println!("Сервер: {}", host);
        println!("Порт: {}", port);
        println!("Секрет: {}", secret);

        println!("\nСсылка для быстрого добавления (откроет Telegram):");
        println!("{}", tg_link);
        println!("\nВеб-ссылка (для копирования в браузер/мессенджер):");
        println!("{}", https_link);

        println!("\nНажмите Enter для остановки...");
        
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        
        println!("Остановка прокси...");
        StopProxy();
        
        println!("Прокси остановлен.");
    } else {
        eprintln!("\nОшибка запуска прокси. Код ошибки: {}", result);
        eprintln!("Возможно, порт {} уже занят другим приложением.", port);
    }
}