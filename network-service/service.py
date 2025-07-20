"""
TCP service for managing network settings.
"""
import socket
import json
import os
from module_networck import get_network_info, set_network_info
from typing import Any

def handle_client(client_socket: socket.socket, connection_name: str) -> None:
    """Handles client connection."""
    try:
        while True:
            data = client_socket.recv(1024)
            if not data:
                break
            try:
                request = json.loads(data.decode())
                request_type = request.get("type")
                if request_type == "GET":
                    network_info = get_network_info(connection_name)
                    response = {"result": "success", "config": network_info}
                elif request_type == "SET":
                    config_data = request.get("config", "")
                    if config_data:
                        set_network_info(config_data, connection_name)
                        response = {"status": "success", "message": "Config is updated"}
                    else:
                        response = {"status": "error", "message": "config key is undefined"}
                else:
                    response = {"status": "error", "message": "Unknown request type"}
            except Exception as e:
                response = {"status": "error", "message": f"Error: {e}"}
            client_socket.send(json.dumps(response).encode())
    except Exception as e:
        print(f"Error handling client: {e}")
    finally:
        client_socket.close()

def main() -> None:
    """Start TCP network management service."""
    host = os.environ.get("SERVER_HOST", "0.0.0.0")
    port = int(os.environ.get("SERVER_PORT", 7575))
    connection_name = os.environ.get("CONNECTION_NAME", 'Wired connection 1')
    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((host, port))
    server.listen(5)
    print(f"Server started on {host}:{port}")
    while True:
        client_socket, client_address = server.accept()
        print(f"Connection from {client_address}")
        handle_client(client_socket, connection_name)

if __name__ == "__main__":
    main()