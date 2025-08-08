
# netsh interface ipv4 set address name="embassy-usb-ethernet" source=static address=5.42.0.61/24 gateway=5.42.0.1 gwmetric=1
# New-NetIPAddress -InterfaceIndex 12 -IPAddress 5.42.0.61 -PrefixLength 24 -DefaultGateway 5.42.0.1


#       set address name="Wired Ethernet Connection" source=static
#       set address "Wired Ethernet Connection" static 10.0.0.9 255.0.0.0 10.0.0.1 1

# netsh interface ipv4 set address name="Embassy-Usb-Ethernet" source=static address=5.42.0.61/24 gateway=5.42.0.1 gwmetric=1 type=unicast store=persistent

import socket

HOST = "fe80::16f8:900c:2409:db60"
#HOST = '10.42.0.61'    # The remote host
PORT = 1234              # The same port as used by the server

with socket.socket(socket.AF_INET6, socket.SOCK_STREAM, 0) as s:
    s.connect((HOST, PORT, 0, 0))
    s.sendall(b'Hello, world\n')
    data = s.recv(1024)
    print('Received', data.decode())