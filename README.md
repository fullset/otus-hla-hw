# otus-hla-hw
ДЗ для курса  Highload Architect. Проект социальной сети.

Разработка велась на fedora 34. Гарантий работоспособности по другие ОС [пока] не предоставлятся :)

Стек:
1. Rust 
2. Redis - хранение сессий
3. Postgres - Основное хранилище

# Зависимости
- rustc 1.72.1 (d5c2e9c34 2023-09-13)
- redis-6.2.6-1.fc34.x86_64
- docker-compose-1.28.6-1.fc34.noarch (для запуска postgresql)

```
$ sudo dnf install redis
$ sudo dnf install docker-compose
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Запуск
## Стартуем Redis
`nohup redis-server &`

## Запуск Postgresql в Docker'е
Для тестовой базы есть настроенный `docker-compose.yaml` файл для запуска одной командой:
```shell
> docker-compose -f postgres/docker-compose.yaml up -d --build
```
Аргумент `--build` нужен, чтобы при обновлении миграций они обновились в контейнере

### Перезапуск контейнера без сохранения состояния
```shell
> cd postgres && docker-compose down && docker-compose build --no-cache && docker-compose up && cd -
```