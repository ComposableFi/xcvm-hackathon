FROM ethereum/client-go:v1.10.19

WORKDIR /app
ADD . /app

VOLUME "/data"
RUN chmod u+x /app/entrypoint.sh
ENTRYPOINT "/app/entrypoint.sh"
EXPOSE 8178 8546
