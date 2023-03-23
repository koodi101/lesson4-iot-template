# lesson4-iot-template

Template code for reading RuuviTag sensor data (temperature, humidity, etc.) with a Raspberry Pi.

NodeJS and NPM should be installed on the system.

## Objectives

- Get a shell on the Raspberry Pi with SSH
- Get the data flowing from the RuuviTag to the Node application
- Add `node-fetch` as a dependency and use `fetch` to POST data to your group's chat server

  > For example sending a message like `{"message": "temperature is ..."}`
- Once the project's backend in running, change the request endpoint and body to send numerical data

## Task 1: Fork and clone

Fork this repository (one per group) and clone it on the Raspberry Pi.

## Task 2: Preparing the RuuviTag

Open the RuuviTag enclosure and remove the insulation from the battery.

## Task 3: Install dependencies and run the application

```
npm install
npm start
```

You should see data being broadcasted by the RuuviTags appear in the output.

## Task 4: Change the request endpoint and body

By default the code in `src/main.js` sends data to the teacher's chat server which you
can read by sending a `GET` request to:

```
http://95.216.154.69:9000/api/chats
```

Change the endpoint to connect to your group's server.

Once you have deployed the project backend, you can change the body to
send the temperature, pressure, etc. as numerical values:

``` json
{
    temperature: data.temperature,
    humidity: data.humidity,
    pressure: data.pressure
}
```
