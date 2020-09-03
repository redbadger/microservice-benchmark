FROM openjdk:8

COPY ./target/rest-service-0.0.1-SNAPSHOT.jar /app.jar

EXPOSE 8000
CMD ["java", "-jar", "/app.jar", "--server.port=8000"]
