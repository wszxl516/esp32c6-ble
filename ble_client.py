#!/usr/bin/python
import argparse
import asyncio
import sys
from bleak import BleakClient, BleakScanner
from bleak.backends.characteristic import BleakGATTCharacteristic
import time

UART_UUID = "6E400001-B5A3-F393-E0A9-E50E24DCCA9E"
UART_RX_UUID = "6E400002-B5A3-F393-E0A9-E50E24DCCA9E"
UART_TX_UUID = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E"
hsv_gradient = bytearray([0x00,  # start hue
                          0xff,  # end huw
                          0xff,  # sat
                          0x0a,  # val
                          0x20,  # sleep interval
                          ])
rgb_gradient = bytearray([0x00,  # start
                          0x0f,  # end
                          0x00,  # 0 red, 1 green, 2 blue
                          0x60,  # sleep interval
                          ])

device_address = "40:4C:CA:57:45:8E"


async def send_data(addr, service_id, characteristic_id, data: bytearray):
    async with BleakClient(addr) as client:
        service = client.services.get_service(service_id)
        characteristic = service.get_characteristic(characteristic_id)
        await client.write_gatt_char(characteristic, data, response=False)


async def led_set(address):
    await send_data(address, "ffff", "fff1", hsv_gradient)


async def time_sync(address):
    t = time.time()
    t = bytearray(int(t).to_bytes(length=8, byteorder="little"))
    await send_data(address, "fff0", "fff1", bytearray(t))


async def uart_terminal(address):
    device = await (BleakScanner
                    .find_device_by_filter(lambda d, a: UART_UUID.lower() in a.service_uuids or d.address == address))

    if device is None:
        print("no matching device found.")
        sys.exit(1)

    def on_disconnect(_: BleakClient):
        print("Device was disconnected, goodbye.")
        for task in asyncio.all_tasks():
            task.cancel()
        sys.exit(0)

    def handle_tx(_: BleakGATTCharacteristic, receive_data: bytearray):
        print(receive_data.decode())

    try:
        async with BleakClient(device, disconnected_callback=on_disconnect) as client:
            await client.start_notify(UART_TX_UUID, handle_tx)
            print("Connected, start typing and press ENTER to send")
            loop = asyncio.get_running_loop()
            uart = client.services.get_service(UART_UUID)
            rx_char = uart.get_characteristic(UART_RX_UUID)
            while True:
                data = await loop.run_in_executor(None, input)
                if not data or data.__len__() > rx_char.max_write_without_response_size:
                    break
                await client.write_gatt_char(rx_char, bytearray(data[0:data.__len__()].encode()), response=False)

    except KeyboardInterrupt:
        sys.exit(0)


def main():
    parser = argparse.ArgumentParser(
        description="settings with esp32c6 ble."
    )
    parser.add_argument(
        "-l",
        "--led",
        required=False,
        dest="led",
        action='store_true',
        help="set led color."
    )
    parser.add_argument(
        "-s",
        "--time-sync",
        action='store_true',
        dest="time_sync",
        help="sync time"
    )
    parser.add_argument(
        "-t",
        "--terminal",
        action='store_true',
        dest="terminal",
        help="uart terminal"
    )
    parser.add_argument(
        "-a",
        "--address",
        type=str,
        dest="address",
        default=device_address,
        help="device ble address"
    )
    args = parser.parse_args()
    try:
        if args.terminal:
            asyncio.run(uart_terminal(args.address))
        elif args.time_sync:
            asyncio.run(time_sync(args.address))
        elif args.led:
            asyncio.run(led_set(args.address))
    except asyncio.CancelledError:
        pass


if __name__ == "__main__":
    main()
