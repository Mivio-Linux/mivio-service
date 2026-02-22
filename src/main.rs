use std::path::Path;

const ETC_SV_DIR: &str = "/etc/sv/";
const VAR_SERVICE_DIR: &str = "/var/service/";
fn main() -> Result<(), std::io::Error> {
    let print_usage = || {
        print!(
"USAGE:
    ms add [SERVICE]     -  enable service
    ms remove [SERVICE]  -  disable service
    ms list              -  list all services
    ms add-list          -  list all enabled services"
        ) //функция, где пишем сообщение как использовать
    };
    let arguments: Vec<String> = std::env::args().collect(); // парсим аргументы
    if arguments.len() == 2 {
        if arguments[1] == "list" {
            list_all_services()?
        }
        // если 2 аргумента и 2аргумент равен list то пишем все сервисы
        else if arguments[1] == "add-list" {
            list_enabled_services()?
        }
    } else if arguments.len() == 3 {
        // иначе если 3 аргумента И
        if arguments[1] == "add" {
            // если 2 аргумент add то
            add_service(&arguments[2])?; //берем 3 аргумент и добавляем сервис
        } else if arguments[1] == "remove" {
            // если 2 аргумент remove ещ
            remove_service(&arguments[2])? // удаляем сервис
        } else {
            print_usage() // иначе выведи как использовать
        }
    } else {
        print_usage() // если не 2 и не 3 аргумента то выведи использование
    }
    Ok(())
}
fn list_dir(s: &str) -> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();
    let read_dir= std::fs::read_dir(s)?;
    for entry in read_dir {
        let entry = entry?;
        if let Some(name) = entry.file_name().into_string().ok() {
            result.push(name);
        }
    }
    Ok(result)
}
fn add_service(service: &String) -> Result<(), std::io::Error> {
    println!("Enabling {service}..."); //пишем что добавляем сервис
    let files: Vec<String> = list_dir(ETC_SV_DIR)?; // смотрим все объекты внутри /etc/sv
    let services: Vec<String> = list_dir(VAR_SERVICE_DIR)?;
    // если файл с названием сервиса есть в дир1 но нет в дир2 то
    if files.contains(service) && !services.contains(service) {
        let path = Path::new(ETC_SV_DIR).join(service);
        let link_path = Path::new(VAR_SERVICE_DIR).join(service); //делаем путь куда будет копироватся
        std::os::unix::fs::symlink(path, link_path)?; //копируем
        println!("Success!");
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "[ERROR] service already added",
        ))
    }
}
fn remove_service(service: &String) -> Result<(), std::io::Error> {
    println!("Disabling {service}...");
    let services: Vec<String> = list_dir(VAR_SERVICE_DIR)?;
    if services.contains(service) {
        let mut path = String::new();
        path.push_str("/var/service/");
        path.push_str(service);
        std::fs::remove_file(path)?;
        println!("Success!");
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("No service {service}"),
        ))
    }
}
fn list_all_services() -> Result<(), std::io::Error> {
    let services: Vec<String> = list_dir(ETC_SV_DIR)?;
    for service in services {
        println!("{service}");
    }
    Ok(())
}
fn list_enabled_services() -> Result<(), std::io::Error> {
    let services: Vec<String> = list_dir(VAR_SERVICE_DIR)?;
    for service in services {
        println!("{service}");
    }
    Ok(())
}
