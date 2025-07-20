import socket
import json
from oled_module import display 
import os

os.environ.setdefault("SERVER_ADDRESS", "0.0.0.0")
os.environ.setdefault("SERVER_PORT", "5050")

os.environ.setdefault("DISPLAY_PORT", "1")
os.environ.setdefault("DISPLAY_ADDRESS", "0x3C")

server_address = (os.environ.get("SERVER_ADDRESS"), int(os.environ.get("SERVER_PORT")))

display_port = int(os.environ.get("DISPLAY_PORT", "1"))
display_address_str = (os.environ.get("DISPLAY_ADDRESS", "0x3C"))
if display_address_str.startswith("0x"):
    display_address_str = display_address_str[2:]

display_address_int = int(display_address_str, 16)

server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

server_socket.bind(server_address)

server_socket.listen(2)
print("Ожидание подключения клиента...")

while True:
    client_socket, client_address = server_socket.accept()
    print(f"Подключение клиента с адресом {client_address}")

    try:
        while True:
            data = client_socket.recv(1024)
            if not data:
                break

            try:
                json_data = json.loads(data.decode('utf-8'))
                print("Получен JSON от клиента:")
                expected_keys = ["gps", "ntp", "time"]
                missing_keys = [key for key in expected_keys if key not in json_data]
                if missing_keys:
                    print(f"Отсутствующие ключи: {', '.join(missing_keys)}")
                    error_response = {
                    "status": "error",
                    "message": "Ошибка при декодировании JSON"
                     }
                    client_socket.send(json.dumps(error_response).encode('utf-8'))

                else:
                    print("Все ожидаемые ключи присутствуют в JSON-объекте.")
                    print(json.dumps(json_data, indent=4))
                    try:
                        display(display_port, display_address_int, json_data)
                        success_response = {
                        "status": "success",
                        "message": "Команда успешно выполнена"
                        }
                        client_socket.send(json.dumps(success_response).encode('utf-8'))
                    except:
                        print(f"Ошибка при отображении на дисплее")
                        error_response = {
                        "status": "error",
                        "message": "Ошибка при отображении"
                        }
                        client_socket.send(json.dumps(error_response).encode('utf-8'))
            except json.JSONDecodeError as e:
                print(f"Ошибка при декодировании JSON: {e}")
                error_response = {
                    "status": "error",
                    "message": "Ошибка при декодировании JSON"
                }
                client_socket.send(json.dumps(error_response).encode('utf-8'))

    except Exception as e:
        print(f"Ошибка при обработке данных: {e}")

    finally:
        client_socket.close()

server_socket.close()