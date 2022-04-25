package tech.artcoded.poc.jms.artemis;

import org.apache.camel.ExchangePattern;
import org.apache.camel.builder.RouteBuilder;
import org.apache.camel.model.dataformat.JsonLibrary;
import org.apache.commons.lang3.RandomStringUtils;
import org.springframework.stereotype.Component;

@Component
public class NotifyRouteBuilder extends RouteBuilder {
  @Override
  public void configure() throws Exception {
    from("jms:topic:{{application.topic.subscription}}")
        .log("receive message ${body}")
        .transform().exchange(exchange -> Message.builder()
            .timestamp(System.currentTimeMillis())
            .randomText(RandomStringUtils.randomAlphanumeric(10, 20))
            .build())
        .marshal().json(JsonLibrary.Jackson)
        .to(ExchangePattern.InOnly, "jms:topic:{{application.topic.publish}}");
  }
}
