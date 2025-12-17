# Python Modules
import requests
import itertools
import logging

from datetime import datetime

#Third party Modules
import polars as pl
import numpy as np


#Constants

URL_CITYBIKE = "http://api.citybik.es/v2/networks/ecobici"

URL_OPENMETEO = "https://air-quality-api.open-meteo.com/v1/air-quality"

URI_POSTGRES = 'postgresql://data_engineer:password123@host.docker.internal:5432/urban_db'

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)

LOGGER = logging.getLogger(__name__)


def read_API(url:str) -> list:
    # Define the API endpoint URL
    LOGGER.info(f"Reading API: {url}")

    try:
        # Make the GET request
        response = requests.get(url)
    
        # Check if the request was successful (status code 200)
        if response.status_code == 200:
            # Parse the JSON response into a Python dictionary
            data = response.json()

            LOGGER.info(f"Completed Read Succesfully")

            return data
        else:
            LOGGER.error(f"Error: API returned status code {response.status_code}")
    
    except requests.exceptions.RequestException as e:
        LOGGER.error(f"An error occurred during the request: {e}")


def extracting_citybikes(data:dict) -> dict:

    LOGGER.info("Extracting Citybikes data...")
    
    data = data['network']['stations']
    processed_data = []
    for item in data:
        filtered_item = {
            'id':item['id'], 
            'name':item['name'], 
            'latitude':item['latitude'], 
            'longitude':item['longitude'], 
            'free_bikes':item['free_bikes'],
            'total_slots':item['extra']['slots'],
            'empty_slots':item['empty_slots'] 
        }

        processed_data.append(filtered_item)
    
    LOGGER.info(f"Extracted {len(processed_data)} rows")

    return processed_data


def fit_circle_least_squares(points):
    """
    Calcula el centro y el radio de un círculo que mejor se ajusta a un conjunto de puntos (x, y)
    utilizando el método de mínimos cuadrados.
    """
    x = points[:, 0]
    y = points[:, 1]
    
    # Ecuación general de un círculo: x^2 + y^2 + 2gx + 2fy + c = 0
    # Transformada para ajuste lineal: z = -2gx - 2fy - c, donde z = x^2 + y^2
    # Creamos la matriz A y el vector B para resolver A * p = B, donde p = [g, f, c]
    A = np.column_stack([-2 * x, -2 * y, np.ones(len(x))])
    B = -(x**2 + y**2)

    # Resolvemos para los parámetros p = [g, f, c] usando numpy.linalg.lstsq
    # rcond=None es para compatibilidad con versiones futuras de numpy
    p, residuals, rank, singular_values = np.linalg.lstsq(A, B, rcond=None)
    
    g, f, c = p
    
    # El centro (xc, yc) y el radio (r) se derivan de los parámetros g, f, c
    xc = g
    yc = f
    
    return xc, yc


def comprobacion_postgres(query, table, df_compared, col_left, col_right):
    
    LOGGER.info(f"Executing {query}...")
    
    df = pl.read_database_uri(
        query,
        URI_POSTGRES,
        engine="connectorx",
    )

    LOGGER.info(f"Obtaining inexisting data...")

    df_filtrado_no_existente = df_compared.join(
        df, 
        left_on=col_left, 
        right_on=col_right,
        how='anti'
    )

    if len(df_filtrado_no_existente)>0:
        
        LOGGER.info(f"Adding {len(df_filtrado_no_existente)} rows to {table}...")

        df_filtrado_no_existente.write_database(
            table_name=f"project_schema.{table}",
            connection=URI_POSTGRES,
            engine="adbc",
            if_table_exists='append'
        )

    else:

        LOGGER.info(f"There is no new data to add to {table}")


def main():
    # Obtención data de Citybike

    data_citybike = read_API(URL_CITYBIKE)
    data_citybike = extracting_citybikes(data_citybike)
    df_stations = pl.DataFrame(data_citybike)
    
    LOGGER.info(f"Rows from Citybike: {len(df_stations)}")

    # Calculo del centro de las estaciones

    puntos_list = []

    for lat,lon in zip(df_stations['latitude'],df_stations['longitude']):
        puntos_list.append([lat,lon])

    puntos_np = np.array(puntos_list)
    centro_x, centro_y = fit_circle_least_squares(puntos_np)

    ## Obtención data de Open-meteo

    url = f'{URL_OPENMETEO}?latitude={centro_x}&longitude={centro_y}&current=pm10,pm2_5'
    
    data = read_API(url)

    df_stations = df_stations.with_columns(
        pl.lit(data['current']['pm10']).alias("pm10"), 
        pl.lit(data['current']['pm2_5']).alias("pm2_5"),
        pl.lit(data['current']['time']).alias("timestamp")
    )

    LOGGER.info(f"Succesfully added 'pm10', 'pm2_5' and 'time' to Citybike Dataframe")

    ## Tratado de datos

    LOGGER.info("Casting 'timestamp' to 'datetime'...")

    df_stations = df_stations.with_columns(
        pl.col('timestamp').str.to_datetime(format="%Y-%m-%dT%H:%M")
    )

    LOGGER.info("Succesfully casted 'timestamp' to 'datetime'")

    LOGGER.info("Calculating 'utilization_rate'...")

    df_stations = df_stations.with_columns(
        ((pl.col('total_slots') - pl.col('empty_slots')) / pl.col('total_slots')).alias('utilization_rate')
    )

    LOGGER.info("Succesfully calculated and added 'utilization_rate'")

    LOGGER.info("Dropping 'nulls'...")

    df_stations = df_stations.drop_nulls(subset=df_stations.columns)

    LOGGER.info("'nulls' dropped")

    ## Creating DF to send to Postgres

    #dim_station: (id, name, latitude, longitude)
    df_dim_station = df_stations.select(
        pl.col("id"),
        pl.col("name"),
        pl.col("latitude"),
        pl.col("longitude")
    )

    query = f"SELECT id FROM project_schema.dim_station"
    comprobacion_postgres(
        query, 
        "dim_station", 
        df_dim_station, 
        "id", 
        "id")

    #fact_measurements: (station_id, timestamp, free_bikes, utilization_rate, pm10_level, pm2_5_level).
    df_fact_measurements = df_stations.select(
        pl.col("id").alias("station_id"),
        pl.col("timestamp"),
        pl.col("free_bikes").cast(pl.Int32),
        pl.col("utilization_rate").cast(pl.Float32),
        pl.col("pm10").cast(pl.Float32).alias("pm10_level"),
        pl.col("pm2_5").cast(pl.Float32).alias("pm2_5_level")
    )

    current_time = df_fact_measurements['timestamp'][0]
    query = f"SELECT station_id, timestamp FROM project_schema.fact_measurements WHERE timestamp = '{current_time}'"
    comprobacion_postgres(
        query, 
        "fact_measurements", 
        df_fact_measurements, 
        ["station_id", "timestamp"], 
        ["station_id", "timestamp"]
    )

    LOGGER.info("Process finished")


if __name__ == '__main__':
    main()