# Python Modules
import requests
import logging
from datetime import datetime

# Third party Modules
import polars as pl
import numpy as np

# Logging Config (CRITICO PARA VER OUTPUT)
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
LOGGER = logging.getLogger(__name__)

# Constants
URL_CITYBIKE = "http://api.citybik.es/v2/networks/ecobici"
URL_OPENMETEO = "https://air-quality-api.open-meteo.com/v1/air-quality"
URI_POSTGRES = 'postgresql://data_engineer:password123@host.docker.internal:5432/urban_db'

def read_API(url:str) -> dict: # Type hint corregido a dict
    LOGGER.info(f"Reading API: {url}")
    try:
        response = requests.get(url)
        if response.status_code == 200:
            LOGGER.info(f"Completed Read Succesfully")
            return response.json()
        else:
            LOGGER.error(f"Error: API returned status code {response.status_code}")
            return {} # Retornar vacío para evitar crash
    except requests.exceptions.RequestException as e:
        LOGGER.error(f"An error occurred during the request: {e}")
        return {}

def extracting_citybikes_polars(data: dict) -> pl.DataFrame:
    LOGGER.info("Extracting Citybikes data...")
    # Polars nativo: Mucho más rápido que iterar listas
    df = pl.DataFrame(data['network']['stations'])
    
    # Sacamos 'slots' del json anidado 'extra'
    # Nota: Si 'extra' a veces es null, esto maneja el error mejor que un dict access directo
    df = df.unnest("extra") 
    
    df = df.select([
        pl.col('id'), 
        pl.col('name'), 
        pl.col('latitude'), 
        pl.col('longitude'), 
        pl.col('free_bikes'),
        pl.col('slots').alias('total_slots'), # Asumiendo que 'slots' venía dentro de 'extra'
        pl.col('empty_slots') 
    ])
    LOGGER.info(f"Extracted {len(df)} rows")
    return df

def fit_circle_least_squares(points:np.array) -> tuple[float, float]:
    x = points[:, 0]
    y = points[:, 1]
    A = np.column_stack([-2 * x, -2 * y, np.ones(len(x))])
    B = -(x**2 + y**2)
    p, _, _, _ = np.linalg.lstsq(A, B, rcond=None)
    g, f, _ = p
    return g, f

def comprobacion_postgres(query:str, table:str, df_compared:pl.DataFrame, col_left:str, col_right:str) -> None:
    LOGGER.info(f"Checking existing data for {table}...")
    try:
        df_existing = pl.read_database_uri(query, URI_POSTGRES, engine="connectorx")
        
        df_new = df_compared.join(
            df_existing, 
            left_on=col_left, 
            right_on=col_right,
            how='anti'
        )

        if len(df_new) > 0:
            LOGGER.info(f"Adding {len(df_new)} rows to {table}...")
            df_new.write_database(
                table_name=f"project_schema.{table}",
                connection=URI_POSTGRES,
                engine="adbc",
                if_table_exists='append'
            )
        else:
            LOGGER.info(f"No new data for {table}")
    except Exception as e:
        LOGGER.error(f"Error in DB check for {table}: {e}")

def main():
    # 1. Citybike Data
    data_citybike = read_API(URL_CITYBIKE)
    if not data_citybike: return # Salir si falló API

    df_stations = extracting_citybikes_polars(data_citybike)
    
    # 2. Geometric calc (Optimizado con to_numpy)
    # Convertimos directo de Polars a NumPy sin loops
    points = df_stations.select(['latitude', 'longitude']).to_numpy()
    centro_x, centro_y = fit_circle_least_squares(points)

    # 3. Open-Meteo Data
    url_weather = f'{URL_OPENMETEO}?latitude={centro_x}&longitude={centro_y}&current=pm10,pm2_5'
    data_weather = read_API(url_weather)
    
    # Enriquecimiento
    df_stations = df_stations.with_columns(
        pl.lit(data_weather['current']['pm10']).cast(pl.Float32).alias("pm10"), 
        pl.lit(data_weather['current']['pm2_5']).cast(pl.Float32).alias("pm2_5"),
        pl.lit(data_weather['current']['time']).alias("timestamp")
    )

    # 4. Transformaciones
    df_stations = df_stations.with_columns(
        pl.col('timestamp').str.to_datetime(format="%Y-%m-%dT%H:%M"),
        ((pl.col('total_slots') - pl.col('empty_slots')) / pl.col('total_slots'))
        .cast(pl.Float32).alias('utilization_rate')
    ).drop_nulls()

    # 5. Carga DIM_STATION
    # IMPORTANTE: Casting explícito aquí también para evitar error de ADBC
    df_dim_station = df_stations.select(
        pl.col("id").str.slice(0, 32),
        pl.col("name"),
        pl.col("latitude").cast(pl.Float32),
        pl.col("longitude").cast(pl.Float32)
    ).unique(subset=["id"]) # Evitar duplicados internos

    comprobacion_postgres(
        "SELECT id FROM project_schema.dim_station", 
        "dim_station", 
        df_dim_station, 
        "id", 
        "id"
    )

    # 6. Carga FACT_MEASUREMENTS
    df_fact = df_stations.select(
        pl.col("id").str.slice(0, 32).alias("station_id"),
        pl.col("timestamp"),
        pl.col("free_bikes").cast(pl.Int32),
        pl.col("utilization_rate"), # Ya casteado arriba
        pl.col("pm10").alias("pm10_level"), # Ya casteado arriba
        pl.col("pm2_5").alias("pm2_5_level") # Ya casteado arriba
    )

    # Formato seguro de fecha para el query
    ts_val = df_fact['timestamp'][0] # Objeto datetime
    ts_str = ts_val.strftime('%Y-%m-%d %H:%M:%S') # Sin microsegundos si API no trae, o ajustar.
    
    # Nota: Open-Meteo suele dar tiempos redondos (ej 14:00), sin micros.
    
    query_fact = f"SELECT station_id, timestamp FROM project_schema.fact_measurements WHERE timestamp = '{ts_str}'"
    
    comprobacion_postgres(
        query_fact, 
        "fact_measurements", 
        df_fact, 
        ["station_id", "timestamp"], 
        ["station_id", "timestamp"]
    )

    LOGGER.info("Pipeline Finished Successfully")

if __name__ == '__main__':
    main()