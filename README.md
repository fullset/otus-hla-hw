# otus-hla-hw
ДЗ для курса  Highload Architect. Проект социальной сети.

Разработка велась на ubuntu 22.04. Гарантий работоспособности по другие ОС [пока] не предоставлятся :)

Стек:
1. Rust 
2. Postgres - Основное хранилище

# Зависимости
- rustc >=1.73.0 (cc66ad468 2023-10-03)
- docker-compose 1.25.0-1 (для запуска postgresql) (Установить по инструкции с https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04)
- libssl-dev (1.1.1f-1ubuntu2.20)

```
$ sudo apt install libssl-dev
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Сборка
```
$ cargo build
```
## Запуск тестов
```
$ cargo test
```
# Запуск
## Запуск Postgresql в Docker'е
Для тестовой базы есть настроенный `docker-compose.yaml` файл для запуска одной командой:
```shell
$ docker-compose -f postgres/docker-compose.yaml up -d --build
```
Аргумент `--build` нужен, чтобы при обновлении миграций они обновились в контейнере

### Перезапуск контейнера без сохранения состояния
```shell
$ cd postgres && docker-compose down && docker-compose build --no-cache && docker-compose up && cd -
```

## Запуск сервиса 
```
$ cargo build
$ ./target/debug/otus-hla-hw --config cfg.yaml
```