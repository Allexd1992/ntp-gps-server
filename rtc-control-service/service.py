"""
TCP service for managing hardware clock (RTC).
"""
import socket
import json
import os
from rtc_module import get_time, set_time
from typing import Any

def handle_client(client_socket: socket.socket, device_path: str) -> None:
    """Handles client connection."""
    try:
        while True:
            data = client_socket.recv(1024)
            if not data:
                break
            try:
                json_data = json.loads(data.decode('utf-8'))
                cmd = json_data.get("cmd")
                if cmd == "get":
                    timestamp = get_time(device_path)
                    response = {"status": "success", "timestamp": timestamp}
                elif cmd == "set":
                    new_timestamp = json_data.get("ts")
                    if new_timestamp is not None:
                        set_time(float(new_timestamp), device_path)
                        response = {"status": "success", "message": "time is updated"}
                    else:
                        response = {"status": "error", "message": "Key 'ts' is not exist"}
                else:
                    response = {"status": "error", "message": "Unknown command"}
            except Exception as e:
                response = {"status": "error", "message": f"Error: {e}"}
            client_socket.send(json.dumps(response).encode('utf-8'))
    except Exception as e:
        print(f"Error processing data: {e}")
    finally:
        client_socket.close()

def main() -> None:
    """Start TCP RTC management service."""
    os.environ.setdefault("SERVER_ADDRESS", "0.0.0.0")
    os.environ.setdefault("SERVER_PORT", "6060")
    os.environ.setdefault("DEVICE_PATH", "/dev/rtc1")
    device_path = os.environ.get("DEVICE_PATH")
    server_address = (os.environ.get("SERVER_ADDRESS"), int(os.environ.get("SERVER_PORT")))
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.bind(server_address)
    server_socket.listen(2)
    print("Waiting for client connection...")
    while True:
        client_socket, client_address = server_socket.accept()
        print(f"Client connection from {client_address}")
        handle_client(client_socket, device_path)

if __name__ == "__main__":
    main()
