# lesson4-iot-template

Template code for reading RuuviTag sensor data (temperature, humidity, etc.) with a Raspberry Pi.

NodeJS and NPM should be installed on the system.

## Objectives

- Get a shell on the Raspberry Pi with SSH
- Start the RuuviTag listener

## Task 1: SSH

Connect to the Raspberry Pi using SSH from your computer:

```shell
ssh pi@1.3.3.7
```

Where you replace `1.3.3.7` with the internal IP address of your group's Raspberry Pi.

You can open a shared tmux session in your group by

1. One member runs the `tmux` command
2. Other members run `tmux attach` to share the terminal.

To detach from the tmux session use `Ctrl-B Ctrl-D`.

## Task 2: Preparing the RuuviTag

Open the RuuviTag enclosure and remove the insulation from the battery.

## Task 3: Run the application

Change directory to lesson4-iot-template and start the node program.

```
cd lesson4-iot-template
npm start
```

You should see data being broadcasted by the RuuviTags appear in the output.

## Note for teachers

Add this line to crontab to automatically refresh IPs

```shell
* * * * * /usr/bin/curl -X PUT -H "Content-Type: application/json" -d "{\"hostname\":\"$(/usr/bin/hostname)\",\"ip\":\"$(/usr/bin/hostname -I)\"}" http://95.216.207.125:9000/api/ips
```

The to list out the IP addresses using httpie

```shell
http GET http://95.216.207.125:9000/api/ips
```
