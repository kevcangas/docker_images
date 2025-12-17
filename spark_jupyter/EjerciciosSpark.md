# Ejercicios

Iniciación del ambiente:


```python
from pyspark.sql import SparkSession
from pyspark.sql.types import StructType, StructField, StringType, IntegerType, DecimalType, DateType, TimestampType, FloatType 
from pyspark.sql import functions as F
from pyspark.sql.window import Window
import datetime

spark = SparkSession.builder.appName("EjerciciosSpark").getOrCreate()

# --- Esquema y Datos de Departamentos ---
schema_dept = StructType([
    StructField("id_departamento", IntegerType(), True),
    StructField("nombre_dept", StringType(), True)
])
data_dept = [(1, 'Ventas'), (2, 'IT'), (3, 'Recursos Humanos')]
dept_df = spark.createDataFrame(data=data_dept, schema=schema_dept)

# --- Esquema y Datos de Empleados ---
schema_emp = StructType([
    StructField("id_empleado", IntegerType(), True),
    StructField("nombre", StringType(), True),
    StructField("id_departamento", IntegerType(), True),
    StructField("salario", FloatType(), True),
    StructField("fecha_contratacion", DateType(), True)
])
data_emp = [
    (101, 'Ana Torres', 1, 55000.00, datetime.date(2023, 1, 15)),
    (102, 'Carlos Ruiz', 1, 52000.00, datetime.date(2023, 3, 20)),
    (103, 'Maria Lopez', 1, 53000.00, datetime.date(2024, 2, 10)),
    (104, 'David Kim', 2, 70000.00, datetime.date(2022, 7, 30)),
    (105, 'Sofia Chen', 2, 72000.00, datetime.date(2023, 11, 1)),
    (106, 'Luis Gonzalez', 3, 48000.00, datetime.date(2024, 5, 5))
]
emp_df = spark.createDataFrame(data=data_emp, schema=schema_emp)

# --- Esquema y Datos de Ventas ---
schema_ventas = StructType([
    StructField("id_venta", IntegerType(), True),
    StructField("id_producto", IntegerType(), True),
    StructField("id_vendedor", IntegerType(), True),
    StructField("monto", FloatType(), True),
    StructField("fecha_venta", TimestampType(), True)
])
data_ventas = [
    (1, 5001, 101, 150.00, datetime.datetime(2025, 1, 10, 9, 30)),
    (2, 5002, 102, 200.50, datetime.datetime(2025, 1, 12, 11, 15)),
    (3, 5001, 101, 150.00, datetime.datetime(2025, 1, 20, 14, 0)),
    (4, 5003, 103, 50.75, datetime.datetime(2025, 2, 5, 16, 22)),
    (5, 5004, 101, 300.00, datetime.datetime(2025, 2, 15, 10, 5)),
    (6, 5002, 102, 200.50, datetime.datetime(2025, 3, 1, 12, 0)),
    (7, 5005, 103, 120.00, datetime.datetime(2025, 4, 5, 9, 0)),
    (8, 5001, 101, 150.00, datetime.datetime(2025, 4, 10, 11, 30)),
    (9, 5006, 102, 75.00, datetime.datetime(2025, 5, 20, 15, 45)),
    (10, 5003, 101, 50.75, datetime.datetime(2025, 6, 18, 13, 10)),
    (11, 5007, 103, 250.00, datetime.datetime(2025, 7, 7, 10, 0)),
    (12, 5002, 102, 200.50, datetime.datetime(2025, 7, 15, 11, 20)),
    (13, 5004, 101, 300.00, datetime.datetime(2025, 8, 1, 9, 15)),
    (14, 5001, 103, 150.00, datetime.datetime(2025, 10, 10, 14, 30)),
    (15, 5005, 102, 120.00, datetime.datetime(2025, 11, 5, 16, 0))
]
ventas_df = spark.createDataFrame(data=data_ventas, schema=schema_ventas)

print("DataFrames created!")

# (Opcional) Verifiquemos que se crearon
dept_df.show()
emp_df.show()
ventas_df.show()
```

    DataFrames created!


                                                                                    

    +---------------+----------------+
    |id_departamento|     nombre_dept|
    +---------------+----------------+
    |              1|          Ventas|
    |              2|              IT|
    |              3|Recursos Humanos|
    +---------------+----------------+
    
    +-----------+-------------+---------------+-------+------------------+
    |id_empleado|       nombre|id_departamento|salario|fecha_contratacion|
    +-----------+-------------+---------------+-------+------------------+
    |        101|   Ana Torres|              1|55000.0|        2023-01-15|
    |        102|  Carlos Ruiz|              1|52000.0|        2023-03-20|
    |        103|  Maria Lopez|              1|53000.0|        2024-02-10|
    |        104|    David Kim|              2|70000.0|        2022-07-30|
    |        105|   Sofia Chen|              2|72000.0|        2023-11-01|
    |        106|Luis Gonzalez|              3|48000.0|        2024-05-05|
    +-----------+-------------+---------------+-------+------------------+
    
    +--------+-----------+-----------+-----+-------------------+
    |id_venta|id_producto|id_vendedor|monto|        fecha_venta|
    +--------+-----------+-----------+-----+-------------------+
    |       1|       5001|        101|150.0|2025-01-10 09:30:00|
    |       2|       5002|        102|200.5|2025-01-12 11:15:00|
    |       3|       5001|        101|150.0|2025-01-20 14:00:00|
    |       4|       5003|        103|50.75|2025-02-05 16:22:00|
    |       5|       5004|        101|300.0|2025-02-15 10:05:00|
    |       6|       5002|        102|200.5|2025-03-01 12:00:00|
    |       7|       5005|        103|120.0|2025-04-05 09:00:00|
    |       8|       5001|        101|150.0|2025-04-10 11:30:00|
    |       9|       5006|        102| 75.0|2025-05-20 15:45:00|
    |      10|       5003|        101|50.75|2025-06-18 13:10:00|
    |      11|       5007|        103|250.0|2025-07-07 10:00:00|
    |      12|       5002|        102|200.5|2025-07-15 11:20:00|
    |      13|       5004|        101|300.0|2025-08-01 09:15:00|
    |      14|       5001|        103|150.0|2025-10-10 14:30:00|
    |      15|       5005|        102|120.0|2025-11-05 16:00:00|
    +--------+-----------+-----------+-----+-------------------+
    


## Ejercicio 1: Calentamiento (Select y Filter)
Tarea: Obtener el nombre y salario de todos los empleados del departamento de 'IT' que ganan más de $70,000.

Conceptos: select(), join(), filter() (o where()).

Pista: Necesitarás unir emp_df y dept_df.


```python
(emp_df.join(dept_df,emp_df.id_departamento == dept_df.id_departamento,'left')
    .filter((emp_df.salario > 70000) & (dept_df.nombre_dept == 'IT'))
    .select("nombre","salario")
    .show())
```

    +----------+-------+
    |    nombre|salario|
    +----------+-------+
    |Sofia Chen|72000.0|
    +----------+-------+
    


## Ejercicio 2: Agregaciones (Group By y Agg)
Tarea: (Réplica del Ejercicio 1 de SQL) Obtener el monto total de ventas (monto) por cada departamento (nombre_dept).

Conceptos: join() (3 tablas), groupBy(), agg(), sum(), alias().

Pista: from pyspark.sql.functions import sum


```python
(ventas_df.join(emp_df, ventas_df.id_vendedor == emp_df.id_empleado,'left')
    .join(dept_df, emp_df.id_departamento == dept_df.id_departamento,'left')
    .groupBy("nombre_dept")
    .agg(F.sum("monto").alias("monto"))
    .select("monto","nombre_dept")
    .show()
    )
```

                                                                                    

    +------+-----------+
    | monto|nombre_dept|
    +------+-----------+
    |2468.0|     Ventas|
    +------+-----------+
    


## Ejercicio 3: Spark SQL (¡Usa tu conocimiento de SQL!)
Tarea: Resuelve el mismo Ejercicio 2 (ventas totales por departamento), pero esta vez usando spark.sql().

Conceptos: createOrReplaceTempView(), spark.sql().

Pista: Primero debes "registrar" cada DataFrame como una tabla temporal.


```python
dept_df.createOrReplaceTempView("departamentos")
emp_df.createOrReplaceTempView("empleados")
ventas_df.createOrReplaceTempView("ventas")
```


```python
result = (spark.sql("""
    SELECT SUM(ventas.monto) AS monto,departamentos.nombre_dept
    FROM ventas LEFT JOIN empleados ON ventas.id_vendedor = empleados.id_empleado
    LEFT JOIN departamentos ON departamentos.id_departamento = empleados.id_departamento
    GROUP BY departamentos.nombre_dept
                   """))
result.show()
```

                                                                                    

    +------+-----------+
    | monto|nombre_dept|
    +------+-----------+
    |2468.0|     Ventas|
    +------+-----------+
    


## Ejercicio 4: Transformación (Window Functions)
Tarea: (Réplica del Ejercicio 5 de SQL) Añade una columna al DataFrame emp_df llamada ranking_salario_depto que muestre el ranking del salario (de mayor a menor) dentro de su propio departamento.

Conceptos: Window.partitionBy().orderBy(), withColumn(), rank().

Pista: from pyspark.sql.window import Window y from pyspark.sql.functions import rank


```python
windowSpec = Window.orderBy(F.desc("salario"))

(emp_df.withColumn("ranking_salario_depto",F.rank().over(windowSpec))
     .show()
    )
```

    25/11/04 04:49:23 WARN WindowExec: No Partition Defined for Window operation! Moving all data to a single partition, this can cause serious performance degradation.
    25/11/04 04:49:23 WARN WindowExec: No Partition Defined for Window operation! Moving all data to a single partition, this can cause serious performance degradation.
    25/11/04 04:49:23 WARN WindowExec: No Partition Defined for Window operation! Moving all data to a single partition, this can cause serious performance degradation.


    +-----------+-------------+---------------+-------+------------------+---------------------+
    |id_empleado|       nombre|id_departamento|salario|fecha_contratacion|ranking_salario_depto|
    +-----------+-------------+---------------+-------+------------------+---------------------+
    |        105|   Sofia Chen|              2|72000.0|        2023-11-01|                    1|
    |        104|    David Kim|              2|70000.0|        2022-07-30|                    2|
    |        101|   Ana Torres|              1|55000.0|        2023-01-15|                    3|
    |        103|  Maria Lopez|              1|53000.0|        2024-02-10|                    4|
    |        102|  Carlos Ruiz|              1|52000.0|        2023-03-20|                    5|
    |        106|Luis Gonzalez|              3|48000.0|        2024-05-05|                    6|
    +-----------+-------------+---------------+-------+------------------+---------------------+
    


## Ejercicio 5: El Pipeline de ETL (Lectura, Transformación y Escritura)
Este es el ejercicio más importante para un Ingeniero de Datos en AWS.

Tarea: Simular un pipeline que "limpia" la tabla de ventas y la guarda en un formato optimizado (Parquet) y particionado.

Toma el DataFrame ventas_df.

Añade tres nuevas columnas: año (ej. 2025), mes (ej. 1) y dia (ej. 10).

Guarda este nuevo DataFrame en el disco, en formato Parquet, en modo "sobrescribir" (overwrite), y particionado por año y mes.

Conceptos: withColumn(), year(), month(), dayofmonth(), write.format(), write.mode(), write.partitionBy(), save().

Pista: from pyspark.sql.functions import year, month, dayofmonth

Por qué es importante: Esto es exactamente lo que harías en un Job de AWS Glue: leer de S3, transformar con Spark, y escribir de nuevo a S3 particionado para que Amazon Athena pueda consultar los datos de forma rápida y barata.


```python
result = (ventas_df.withColumn("año", F.year("fecha_venta"))
    .withColumn("mes", F.month("fecha_venta"))
    .withColumn("dia", F.dayofmonth("fecha_venta"))     
    )

result.show()

result.write.format("parquet").mode("overwrite").partitionBy("año","mes").save("/opt/spark/work-dir/ejercicio5")
```

    +--------+-----------+-----------+-----+-------------------+----+---+---+
    |id_venta|id_producto|id_vendedor|monto|        fecha_venta| año|mes|dia|
    +--------+-----------+-----------+-----+-------------------+----+---+---+
    |       1|       5001|        101|150.0|2025-01-10 09:30:00|2025|  1| 10|
    |       2|       5002|        102|200.5|2025-01-12 11:15:00|2025|  1| 12|
    |       3|       5001|        101|150.0|2025-01-20 14:00:00|2025|  1| 20|
    |       4|       5003|        103|50.75|2025-02-05 16:22:00|2025|  2|  5|
    |       5|       5004|        101|300.0|2025-02-15 10:05:00|2025|  2| 15|
    |       6|       5002|        102|200.5|2025-03-01 12:00:00|2025|  3|  1|
    |       7|       5005|        103|120.0|2025-04-05 09:00:00|2025|  4|  5|
    |       8|       5001|        101|150.0|2025-04-10 11:30:00|2025|  4| 10|
    |       9|       5006|        102| 75.0|2025-05-20 15:45:00|2025|  5| 20|
    |      10|       5003|        101|50.75|2025-06-18 13:10:00|2025|  6| 18|
    |      11|       5007|        103|250.0|2025-07-07 10:00:00|2025|  7|  7|
    |      12|       5002|        102|200.5|2025-07-15 11:20:00|2025|  7| 15|
    |      13|       5004|        101|300.0|2025-08-01 09:15:00|2025|  8|  1|
    |      14|       5001|        103|150.0|2025-10-10 14:30:00|2025| 10| 10|
    |      15|       5005|        102|120.0|2025-11-05 16:00:00|2025| 11|  5|
    +--------+-----------+-----------+-----+-------------------+----+---+---+
    



```python
df = spark.read.load("/opt/spark/work-dir/ejercicio5/año=2025/mes=1", format="parquet")

df.show()
```

    +--------+-----------+-----------+-----+-------------------+---+
    |id_venta|id_producto|id_vendedor|monto|        fecha_venta|dia|
    +--------+-----------+-----------+-----+-------------------+---+
    |       3|       5001|        101|150.0|2025-01-20 14:00:00| 20|
    |       1|       5001|        101|150.0|2025-01-10 09:30:00| 10|
    |       2|       5002|        102|200.5|2025-01-12 11:15:00| 12|
    +--------+-----------+-----------+-----+-------------------+---+
    

