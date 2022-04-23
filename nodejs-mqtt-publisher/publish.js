const mqtt = require("mqtt");
const faker = require("@faker-js/faker").default;

console.log("running the publisher...");
console.log("will send message to rust through artemis using mqtt protocol");

const CLIENT_ID = process.env.CLIENT_ID || "poc-mqtt-publisher";
const TOPIC = process.env.MQTT_TOPIC_PUBLISHING || "action";
const USERNAME = process.env.MQTT_USERNAME || "root";
const PASSWORD = process.env.MQTT_PASSWORD || "root";
const PORT = process.env.MQTT_PORT || "11883";
const HOSTNAME = process.env.MQTT_HOST || "localhost";
const MAX_MESSAGE = process.env.MAX_MESSAGE || 50;

const client = mqtt.connect(`mqtt://${HOSTNAME}:${PORT}`, {
  clientId: CLIENT_ID,
  username: USERNAME,
  password: PASSWORD,
  clean: true,
});

client.on("connect", () => {
  let count = 0;
  let interval = setInterval(() => {
    let data = {
      username: faker.internet.userName(),
      firstName: faker.name.firstName(),
      lastName: faker.name.lastName(),
      email: faker.internet.email(),
    };
    console.log(`sending `, data);
    client.publish(TOPIC, JSON.stringify(data));
    count+=1;
    if (count === MAX_MESSAGE) {
      console.log('done');
      clearInterval(interval);
    }
  }, 2000);
});
