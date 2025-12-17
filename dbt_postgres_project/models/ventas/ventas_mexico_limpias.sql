{{ config(materialized='table') }}

WITH source_data AS (
    -- 'ref' es la función mágica de dbt para referenciar otras tablas o seeds
    SELECT * FROM {{ ref('ventas_brutas') }}
)

SELECT
    id,
    producto,
    monto,
    -- Vamos a calcular el IVA (16%)
    monto * 0.16 as iva,
    monto * 1.16 as total_con_impuesto,
    CURRENT_DATE as fecha_transformacion
FROM source_data
WHERE 
    pais = 'MX' 
    AND estado = 'completado'