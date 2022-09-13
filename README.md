# Removing duplicate files, recursively removing empty folders

Программа сравнивает две папки и удаляет одинаковые файлы во второй папке (сравнение папок разных версий)

Создается CRC32 сумма для сравниваемых файлов

```bash
cargo build --release
cd ./target/release/
```

```bash
remove_duplicate /папка1 /папка2_в_которой_нужно_удалить_дубликаты
```
