from pyspark.sql import SparkSession
from pyspark.sql.functions import from_json, col
from pyspark.sql.types import StructType, StructField, StringType, DoubleType, LongType

# 1. Iniciar Sesión Spark (con soporte para Kafka)
spark = SparkSession.builder \
    .appName("MonitorIoT") \
    .config("spark.jars.packages", "org.apache.spark:spark-sql-kafka-0-10_2.12:3.5.0") \
    .getOrCreate()

spark.sparkContext.setLogLevel("ERROR") # Reducir ruido en logs

# 2. Definir el esquema de los datos de entrada (JSON)
schema = StructType([
    StructField("sensor_id", StringType()),
    StructField("temperatura", DoubleType()),
    StructField("timestamp", LongType())
])

# 3. Leer Stream desde Kafka
df_raw = spark.readStream \
    .format("kafka") \
    .option("kafka.bootstrap.servers", "kafka:29092") \
    .option("subscribe", "sensores_iot") \
    .option("startingOffsets", "latest") \
    .load()

# 4. Transformación: Convertir bytes a String y luego a JSON estructurado
df_parsed = df_raw.selectExpr("CAST(value AS STRING)") \
    .select(from_json(col("value"), schema).alias("data")) \
    .select("data.*")

# 5. Lógica de Negocio: Filtrar temperaturas altas (Alerta)
df_alertas = df_parsed.filter(col("temperatura") > 30.0)

# 6. Escribir salida a consola
query = df_alertas.writeStream \
    .outputMode("append") \
    .format("console") \
    .option("truncate", "false") \
    .start()

print("Esperando datos... (Presiona Ctrl+C para salir)")
query.awaitTermination()