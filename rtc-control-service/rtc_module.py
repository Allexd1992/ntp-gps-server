"""
Module for working with hardware clock (RTC) via hwclock and date.
"""
import subprocess
from datetime import datetime
from typing import Optional

def get_time(device_path: str) -> Optional[int]:
    """
    Gets current time from RTC device via hwclock.
    Returns timestamp (int) or None on error.
    """
    command = ["hwclock", "--get", "-f", device_path]
    try:
        result = subprocess.run(command, capture_output=True, text=True, check=True)
        stdout_str = result.stdout.strip()
        datetime_obj = datetime.strptime(stdout_str, "%Y-%m-%d %H:%M:%S.%f%z")
        return int(datetime_obj.timestamp())
    except Exception as e:
        print(f"Failed to get RTC time: {e}")
        return None

def set_time(new_timestamp: float, device_path: str) -> None:
    """
    Sets RTC and system time by timestamp.
    """
    if not isinstance(new_timestamp, (int, float)):
        raise ValueError("Invalid timestamp.")
    new_datetime = datetime.fromtimestamp(float(new_timestamp))
    rtc_time_str = new_datetime.strftime("%Y-%m-%d %H:%M:%S")
    try:
        rtc_clock = subprocess.Popen([
            "hwclock", "--systohc", "--date", rtc_time_str, "-f", device_path
        ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        _, rtc_stderr = rtc_clock.communicate()
        if rtc_clock.returncode != 0:
            raise RuntimeError(f"Failed to set RTC time: {rtc_stderr}")
        local_time_str = new_datetime.strftime("%Y-%m-%d %H:%M:%S")
        local_clock = subprocess.Popen([
            "date", "-s", local_time_str
        ], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        _, local_stderr = local_clock.communicate()
        if local_clock.returncode != 0:
            raise RuntimeError(f"Failed to set local time: {local_stderr}")
    except Exception as e:
        print(f"Error setting time: {e}")
        raise
