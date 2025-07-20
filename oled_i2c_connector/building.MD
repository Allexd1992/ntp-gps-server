


```
pip install pyinstaller
pyinstaller --onefile service.py
cd ./dist
chmod +x service
./service
```
 ```
 nano /etc/systemd/system/oled_i2c_connector.service
 
# /etc/systemd/system/oled_i2c_connector.service/oled_i2c_connector.service
[Unit]
Description=Connection Service for OLED Display over I2C
After=network.target

[Service]
ExecStart=/root/Adafruit_Python_SSD1306/orange_pi-ssd1306_oled/dist/service/service
WorkingDirectory=/root/Adafruit_Python_SSD1306/orange_pi-ssd1306_oled/dist/service
RuntimeMaxSec=180s
Restart=always
User=root
Group=root

[Install]
WantedBy=multi-user.target

 ```
 