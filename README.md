# POC Cross plateform AMS

##  Goal

[Apache Artemis](https://activemq.apache.org/components/artemis/documentation/latest/) supports several protocols,
such as AMQP, OpenWire, MQTT & stomp.

[Current monolith](https://github.com/openartcoded/backend) uses a little bit of artemis through its JMS interface.

In order to break the monolith and supports other languages, MQTT can be used as an alternative to interact with the java backend.


## Run the MQTT example

- `docker-compose -f docker-compose.yml -f docker-compose.mqtt.yml up -d` 
- Check the logs of the publisher:

        docker-compose -f docker-compose.yml  \
                    -f docker-compose.mqtt.yml \
                    logs -f mqtt_nodejs_publisher 

- Check the logs of the subscriber: 

        docker-compose -f docker-compose.yml  \
                    -f docker-compose.mqtt.yml \
                    logs -f mqtt_rust_subscriber 


> a json payload from a nodejs backend is sent every 2s to a topic that is consumed by a rust backend.
> the rust backend simply logs the json payload received.

## Test other languages / protocol (AMQP, OpenWire, Stomp, HornetQ)

> Replace `<language>` by the language name you chose in lower case(e.g `csharp`)<br>
  Replace `<protocol>` by the protocol you chose in lower case(e.g `amqp`)<br>
  Replace `<type>` by either `publisher` or `subscriber`

- Create a new branch `git checkout -B feat/<language>-<protocol>`
    - E.g: `git checkout -B feat/csharp-amqp` 

- Create your application under `<language>-<protocol>-<type>`
- If you build a publisher, it should publish json payload  to topic `shared_topic` 
- If you build a subscriber, it should subscribe to topic `shared_topic` and consume json payload
- Locally, the following ports are exposed:

    | Protocol               | Port          |
    | ---------------------- | ------------- |
    | `MQTT`                 | `11883`       |          
    | `HORNETQ/STOMP`        | `15445`       |
    | `AMQP`                 | `15672`       |
    | `STOMP`                | `55513`       |
    | `ALL`                  | `55516`       |

- Credentials for the artemis service : `root` as username & `root`as password.
- Create a docker image of your application
- If it doesn't exist yet, create a `docker-compose.<protocol>.yml`file
- Add your service to it, e.g for a subscriber:

```
  <protocol>_<language>_<type>:
    restart: always
    build: ./<language>-<protocol>-<type>/.
    links:
      - artemis
    environment:
      HOST: artemis
      PORT: <port-protocol>
      USERNAME: root 
      PASSWORD: root 
      TOPIC: shared_topic 
```

## Non soyentific benchmark

```
NAME                               CPU %     MEM USAGE / LIMIT     MEM %      
rust                               0.05%     2.426MiB / 15.06GiB   0.02%
nodejs                             0.00%     75.45MiB / 15.06GiB   0.49%
java                               0.08%     274.9MiB / 15.06GiB   1.78%
```


