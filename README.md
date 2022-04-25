# POC Cross plateform AMS

##  Goal

[Apache Artemis](https://activemq.apache.org/components/artemis/documentation/latest/) supports several protocols,
such as AMQP, OpenWire, MQTT & stomp.

[Current monolith](https://github.com/openartcoded/backend) uses a little bit of artemis through its JMS interface.

In order to break the monolith and supports other languages, MQTT can be used as an alternative to interact with the java backend.

## Test other languages / protocol (AMQP, OpenWire, Stomp, HornetQ)

- Locally, the following ports are exposed:

    | Protocol               | Port          |
    | ---------------------- | ------------- |
    | `MQTT`                 | `11883`       |          
    | `HORNETQ/STOMP`        | `15445`       |
    | `AMQP`                 | `15672`       |
    | `STOMP`                | `55513`       |
    | `ALL`                  | `55516`       |

- Credentials for the artemis service : `root` as username & `root`as password.

## Non soyentific benchmark

```
NAME                               CPU %     MEM USAGE / LIMIT     MEM %      
rust                               0.05%     2.426MiB / 15.06GiB   0.02%
nodejs                             0.00%     75.45MiB / 15.06GiB   0.49%
java                               0.08%     274.9MiB / 15.06GiB   1.78%
```


