"""
TCP service for outputting information to OLED display.
"""
import socket
import json
import os
from oled_module import display
from typing import Any

def handle_client(client_socket: socket.socket, display_port: int, display_address_int: int) -> None:
    """Handles client connection and outputs data to display."""
    try:
        while True:
            data = client_socket.recv(1024)
            if not data:
                break
            try:
                json_data = json.loads(data.decode('utf-8'))
                expected_keys = ["gps", "ntp", "time"]
                missing_keys = [key for key in expected_keys if key not in json_data]
                if missing_keys:
                    response = {"status": "error", "message": "Error decoding JSON"}
                else:
                    try:
                        display(display_port, display_address_int, json_data)
                        response = {"status": "success", "message": "Command executed successfully"}
                    except Exception as e:
                        print(f"Error displaying on screen: {e}")
                        response = {"status": "error", "message": "Display error"}
            except Exception as e:
                print(f"Error decoding JSON: {e}")
                response = {"status": "error", "message": "Error decoding JSON"}
            client_socket.send(json.dumps(response).encode('utf-8'))
    except Exception as e:
        print(f"Error processing data: {e}")
    finally:
        client_socket.close()

def main() -> None:
    """Start TCP service for OLED display."""
    os.environ.setdefault("SERVER_ADDRESS", "0.0.0.0")
    os.environ.setdefault("SERVER_PORT", "5050")
    os.environ.setdefault("DISPLAY_PORT", "1")
    os.environ.setdefault("DISPLAY_ADDRESS", "0x3C")
    server_address = (os.environ.get("SERVER_ADDRESS"), int(os.environ.get("SERVER_PORT")))
    display_port = int(os.environ.get("DISPLAY_PORT", "1"))
    display_address_str = os.environ.get("DISPLAY_ADDRESS", "0x3C")
    if display_address_str.startswith("0x"):
        display_address_str = display_address_str[2:]
    display_address_int = int(display_address_str, 16)
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind(server_address)
    server_socket.listen(2)
    print("Waiting for client connection...")
    while True:
        client_socket, client_address = server_socket.accept()
        print(f"Client connection from {client_address}")
        handle_client(client_socket, display_port, display_address_int)

if __name__ == "__main__":
    main()