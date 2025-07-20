"""
Module for outputting information to OLED I2C display.
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
    Outputs information to OLED I2C display.
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
    font2 = ImageFont.truetype('/root/Adafruit_Python_SSD1306/orange_pi-ssd1306_oled/fonts/C&C Red Alert [INET].ttf', 12)
    with canvas(oled) as draw:
         if eth0_ipv4:
             draw.text((0, 0), f"IPv4: {eth0_ipv4}", font=font2, fill=255)
         if gps:
             draw.text((0, 14), f"From GPS: {gps}", font=font2, fill=255)
         if ntp_srv:
             draw.text((0, 26), f"From SNTP: {ntp_srv}", font=font2, fill=255)
         if time_val:
             draw.text((0, 34), "Actual Time:", font=font2, fill=255)
             draw.text((0, 44), f"{time_val}", font=font2, fill=255)