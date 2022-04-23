FROM maven:3-amazoncorretto-17 as builder
WORKDIR /app
COPY pom.xml .
RUN mvn dependency:resolve
COPY src/ src/
RUN mvn clean install -DskipTests

FROM amazoncorretto:17
WORKDIR /app
COPY --from=builder /app/target/app.jar .
CMD ["java", "-jar", "app.jar"]