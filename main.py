import subprocess
import sys
import os
import secrets

EXECUTABLE_PATH = os.path.join("target", "release", "tgwsproxy.exe")
SECRET_KEY = secrets.token_hex(16)

def main():
    if not os.path.exists(EXECUTABLE_PATH):
        print(f"Ошибка: Файл '{EXECUTABLE_PATH}' не найден!")
        print("Убедитесь, что вы выполнили 'cargo build --release'")
        sys.exit(1)
    
    command = [EXECUTABLE_PATH, "--host", "127.0.0.1", "--port", "1443", "--secret", SECRET_KEY]

    process = None
    try:
        process = subprocess.Popen(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            encoding='utf-8'
            # creationflags=subprocess.CREATE_NO_WINDOW
        )

        print("Запуск прокси, логи от Rust: ")
        print("-" * 50)

        for line in iter(process.stdout.readline, ''):
            if line:
                print(line.strip())
            
            if process.poll() is not None:
                break

    except KeyboardInterrupt:
        print("\n\nПолучен сигнал остановки (Ctrl+C). Завершаем работу...")
    except Exception as e:
        print(f"\nПроизошла непредвиденная ошибка: {e}")
    finally:
        if process and process.poll() is None:
            print("Остановка процесса прокси...")
            process.terminate()
            
            try:
                process.wait(timeout=5)
                print("Прокси корректно остановлен.")
            except subprocess.TimeoutExpired:
                print("Прокси не ответил вовремя. Принудительное завершение...")
                process.kill()
                process.wait()
                print("Процесс принудительно завершен.")
        
        print("Работа скрипта завершена.")

if __name__ == "__main__":
    main()