import time
import board
import digitalio
import paho.mqtt.client as paho
from paho import mqtt

print("starting...")

led1 = digitalio.DigitalInOut(board.D19)
led1.direction = digitalio.Direction.OUTPUT

led2 = digitalio.DigitalInOut(board.D13)
led2.direction = digitalio.Direction.OUTPUT

led3 = digitalio.DigitalInOut(board.D6)
led3.direction = digitalio.Direction.OUTPUT

led4 = digitalio.DigitalInOut(board.D26)
led4.direction = digitalio.Direction.OUTPUT


def on_connect(client, userdata, flags, rc, properties=None):
    print("CONNACK received with code %s." % rc)


def on_message(client, userdata, msg):
    if msg.topic == 'led/1':
        print("toogle 1")
        led1.value = not led1.value
    if msg.topic == 'led/2':
        print("toogle 2")
        led2.value = not led2.value
    if msg.topic == 'led/3':
        print("toogle 3")
        led3.value = not led3.value
    if msg.topic == 'led/4':
        print("toogle 4")
        led4.value = not led4.value

    # print(msg.topic + " " + str(msg.qos) + " " + str(msg.payload))


def on_subscribe(client, userdata, mid, granted_qos, properties=None):
    print("Subscribed: " + str(mid) + " " + str(granted_qos))


def on_publish(client, userdata, mid, properties=None):
    print("mid: " + str(mid))


client = paho.Client(client_id="raspberrypi",
                     userdata=None, protocol=paho.MQTTv5)
client.on_connect = on_connect

client.tls_set(tls_version=mqtt.client.ssl.PROTOCOL_TLS)
client.username_pw_set("iotworkshop", "IotWorkshop1")
client.connect("df24f150d5654c89913485bda5986e90.s1.eu.hivemq.cloud", 8883)

client.on_subscribe = on_subscribe
client.on_message = on_message
client.on_publish = on_publish

client.subscribe("led/#", qos=1)

client.loop_forever()
