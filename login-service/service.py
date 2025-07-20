"""
TCP service for user authentication.
"""
import socket
import json
import os
from module_login import check_password
from typing import Any

def handle_client(client: socket.socket) -> None:
    """Handles client connection."""
    try:
        data = client.recv(1024).decode()
        json_data = json.loads(data)
        if "login" in json_data and "password" in json_data:
            username = json_data["login"]
            password = json_data["password"]
            result = check_password(username, password)
            print(result)
            if result is True:
                response = {"result": "success", "message": "Login and password is correct"}
            else:
                response = {"result": "error", "message": str(result) if isinstance(result, str) else "Authentication failed"}
        else:
            response = {"result": "error", "message": "Invalid JSON format"}
        client.send(json.dumps(response).encode('utf-8'))
    except Exception as e:
        print(f"Error handling client: {e}")
    finally:
        client.close()

def main() -> None:
    """Start TCP authentication service."""
    host = os.environ.get("SERVER_HOST", "0.0.0.0")
    port = int(os.environ.get("SERVER_PORT", 7070))
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((host, port))
    server.listen(1)
    print(f"Server started on {host}:{port}")
    while True:
        client, addr = server.accept()
        print(f"Connection received from {addr}")
        handle_client(client)

if __name__ == "__main__":
    main()