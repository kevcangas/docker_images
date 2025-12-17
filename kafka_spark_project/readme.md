## En la consola local

docker compose up

py /files/producer.py

## En la imagen de Spark

docker compose exec -u 0 spark //opt/spark/bin/spark-submit --packages org.apache.spark:spark-sql-kafka-0-10_2.12:3.5.0 //home/spark/processor.py