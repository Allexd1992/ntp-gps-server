"""
Module for checking Linux user password.
"""
import os
import subprocess
import crypt
from typing import Union

def login_detect(username: str, password: str) -> bool:
    """
    Checks the ability to login as a user via su.
    Returns True if login is successful.
    """
    try:
        command = 'echo "success"'
        output = os.popen(f'echo {password} | su -c "{command}" {username} 2>&1').read()
        return "success" in output
    except Exception as e:
        print(f"Error: {e}")
        return False

def check_password(username: str, input_password: str) -> Union[bool, str]:
    """
    Checks user password against /etc/shadow.
    Returns True if password is correct, otherwise False or error string.
    """
    try:
        result = subprocess.check_output(['sudo', 'grep', f'^{username}:', '/etc/shadow']).decode('utf-8').strip()
        password_hash = result.split(":")[1]
        hashed_input_password = crypt.crypt(input_password, password_hash)
        return hashed_input_password == password_hash
    except subprocess.CalledProcessError:
        return "User does not exist."
    except Exception as e:
        return f"Error: {e}"
