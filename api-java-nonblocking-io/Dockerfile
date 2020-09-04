FROM openjdk:8

RUN useradd svc
WORKDIR /app

COPY --chown=svc:svc ./target/demo-0.0.1-SNAPSHOT.jar app.jar

USER svc

EXPOSE 8000
CMD ["java", "-jar", "app.jar", "--server.port=8000"]
