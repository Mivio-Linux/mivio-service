
const ETC_SV_DIR: &str = "/etc/sv/";
const VAR_SERVICE_DIR: &str = "/var/service/";
fn main() {
    let print_usage = ||{
        print!("USAGE:\n ms add [SERVICE]     -  add service\n ms remove [SERVICE]  -  remove service\n ms list              -  list all services\n") //функция, где пишем сообщение как использовать
    };
    let arguments: Vec<String> = std::env::args().collect(); // парсим аргументы
    if arguments.len() == 2 && arguments[1] == "list".to_string() {  list_services()  }// если 2 аргумента и 2аргумент равен list то пишем все сервисы
    else if arguments.len() == 3 { // иначе если 3 аргумента И
        if arguments[1] == "add" { // если 2 аргумент add то
            add_service(&arguments[2]) //берем 3 аргумент и добавляем сервис
        } else if arguments[1] == "remove"{ // если 2 аргумент remove ещ
            remove_service(&arguments[2]) // удаляем сервис
        } else {
            print_usage() // иначе выведи как использовать
        }
    }
    else {
        print_usage() // если не 2 и не 3 аргумента то выведи использование
    }
}
fn list_dir(s: &str) -> Vec<String>{
    let files: Vec<String> = std::fs::read_dir(s).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect(); // создаем вектор,  в который записываем все имена дочерних объектов
    files
}
fn add_service(service: &String){
    println!("Adding Service..."); //пишем что добавляем сервис
    let files: Vec<String> = list_dir(ETC_SV_DIR); // смотрим все объекты внутри /etc/sv

    let services: Vec<String> = list_dir(VAR_SERVICE_DIR);
    if files.contains(&service) && !services.contains(&service)  { // если файл с названием сервиса есть в дир1 но нет в дир2 то
        let mut path = String::new(); // создаем строку
        path.push_str(ETC_SV_DIR); //добавляем часть путя
        path.push_str(&service); // добавляем вторую, будет /etc/sv/service
        let mut link_path = String::from(VAR_SERVICE_DIR); //делаем путь куда будет копироватся
        link_path.push_str(service);
        let _link = std::os::unix::fs::symlink(path, link_path).expect("[ERROR] can`t add link!"); //копируем
        println!("success!");
    }
    else {
        println!("[ERROR] service already added"); 
    }
}
fn remove_service(service: &String) {
    println!("Removing...");
    let services: Vec<String> = list_dir(VAR_SERVICE_DIR);
    if services.contains(&service) {
        let mut path = String::new();
        path.push_str("/var/service/");
        path.push_str(&service);
        std::fs::remove_file(path).unwrap_or_else(|why| { // удаляем ссылку
        println!("[ERROR] {:?}", why.kind());
    });
}
}
fn list_services() {
    let services: Vec<String> = list_dir(ETC_SV_DIR);
    for service in services {
        println!("{}", service);
    }
}
