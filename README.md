# lesson4-iot-template

Template code for reading RuuviTag sensor data (temperature, humidity, etc.) with a Raspberry Pi.

## Objectives

- Get a shell on the Raspberry Pi with SSH
- Disable SSH password authentication as we have keys
- Get the data flowing from the RuuviTag to the remote backend server

## Step 1: Fork and change IP

These steps should be done only once per group.

- Fork this repository (one per group)
- Clone it
- Change the IP address to point to your remote server

## Step 2: Deployment

- SSH in to the Raspberry Pi (the teacher has more detailed instructions)
  ```shell
  ssh pi@<RPI_IP> -i ./path/to/id_rsa_koodi101rpi
  ```
- Disable password authentication for SSH
  - Edit SSH daemon's configuration as the root user
    ```shell
    sudoedit /etc/ssh/sshd_config
    ```
  - Find the following lines and change them accordingly (remove the # comment)
    ```
    # To disable tunneled clear text passwords, change to no here!
    PasswordAuthentication no
    ```
  - Restart SSH daemon
    ```shell
    sudo systemctl restart ssh
    ```
- To ensure that password authentication is disabled, try to log in with SSH without issuing the `-i ...` option. SSH should not prompt for a password and fail instead.

## Step 3: Preparing the RuuviTag

Open the RuuviTag enclosure and remove the insulation from the battery.

## Step 4: Run the application on the Raspberry [WIP]

TODO Add Rust instructions

TODO Rust takes 1hr to compile the application on a RPi zero

You should see data being broadcasted by the RuuviTags appear in the output.
