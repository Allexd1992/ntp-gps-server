"""
Module for getting and setting network settings via nmcli.
"""
import subprocess
from typing import Dict, Any

def get_network_info(connection_name: str) -> Dict[str, Any]:
    """
    Gets network connection information via nmcli.
    Returns dictionary with keys: address, gateway, dns.
    """
    try:
        command = f'nmcli --terse --fields IP4.ADDRESS,IP4.GATEWAY,IP4.DNS connection show "{connection_name}"'
        output = subprocess.check_output(command, shell=True, universal_newlines=True)
        lines = output.strip().split('\n')
        data_dict = {}
        for line in lines:
            if ':' not in line:
                continue
            key, value = line.split(':', 1)
            data_dict[key.strip()] = value.strip()
        data_obj = {
            'address': data_dict.get('IP4.ADDRESS[1]', ''),
            'gateway': data_dict.get('IP4.GATEWAY', ''),
            'dns': data_dict.get('IP4.DNS[1]', '')
        }
        return data_obj
    except Exception as e:
        raise RuntimeError(f"Error getting network information: {e}")

def set_network_info(config_json: Dict[str, Any], connection_name: str) -> None:
    """
    Sets network settings via nmcli.
    config_json must contain address, gateway, dns.
    """
    try:
        command = (
            f'sudo nmcli connection modify "{connection_name}" '
            f'ipv4.method manual ipv4.addresses "{config_json["address"]}" '
            f'ipv4.gateway "{config_json["gateway"]}" ipv4.dns "{config_json["dns"]}"'
        )
        subprocess.check_output(command, shell=True, universal_newlines=True)
        save_command = f'sudo nmcli connection up "{connection_name}"'
        subprocess.check_output(save_command, shell=True, universal_newlines=True)
    except Exception as e:
        raise RuntimeError(f"Error setting network configuration: {e}")