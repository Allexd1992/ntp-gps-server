"""
Module for outputting information to OLED display.
"""
import datetime
from oled.device import ssd1306
from oled.render import canvas
from PIL import ImageFont
import netifaces as ni
from typing import Optional, Dict, Any

def get_ipv4_address(interface_name: str) -> Optional[str]:
    """
    Gets IPv4 address of interface.
    """
    try:
        address = ni.ifaddresses(interface_name)[ni.AF_INET][0]['addr']
        return address
    except Exception as e:
        print(f"Error occurred: {e}")
        return None

def display(port: int, address: int, json_data: Dict[str, Any]) -> None:
    """
    Outputs information to OLED display.
    json_data can contain keys: gps, ntp, time.
    """
    oled = ssd1306(port=port, address=address)
    eth0_ipv4 = get_ipv4_address('eth0')
    gps = json_data.get("gps", "")
    ntp_srv = json_data.get("ntp", "")
    time_val = json_data.get("time", "")
    current_datetime = datetime.datetime.now()
    local_time = current_datetime.strftime("%Y-%m-%d %H:%M:%S")
    font = ImageFont.load_default()
    try:
        font2 = ImageFont.truetype('/root/Adafruit_Python_SSD1306/orange_pi-ssd1306_oled/fonts/C&C Red Alert [INET].ttf', 12)
    except Exception:
        font2 = font
    with canvas(oled) as draw:
        y = 0
        if eth0_ipv4:
            draw.text((0, y), f"IPv4: {eth0_ipv4}", font=font2, fill=255)
            y += 14
        if gps:
            draw.text((0, y), f"From GPS: {gps}", font=font2, fill=255)
            y += 12
        if ntp_srv:
            draw.text((0, y), f"From SNTP: {ntp_srv}", font=font2, fill=255)
            y += 12
        if time_val:
            draw.text((0, y), "Actual Time:", font=font2, fill=255)
            y += 10
            draw.text((0, y), f"{time_val}", font=font2, fill=255)