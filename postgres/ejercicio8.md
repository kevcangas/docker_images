Tarea: Encuentra a todos los vendedores que hayan realizado una venta en tres o más días consecutivos. Muestra el id_vendedor, el dia_inicio_racha y el dia_fin_racha.

Concepto: Este es un problema clásico de "Gaps and Islands". Demuestra un dominio muy avanzado de SQL y funciones de ventana.

Pista:

Usa DENSE_RANK() sobre fecha_venta (particionado por vendedor).

Resta el DENSE_RANK() de la fecha (fecha_venta - INTERVAL '1' DAY * DENSE_RANK()). Esto creará un "grupo" constante para cada racha consecutiva.

Agrupa por ese "grupo" y el id_vendedor para encontrar el MIN(fecha_venta) y MAX(fecha_venta) de cada racha.

Filtra donde el conteo de días en la racha sea >= 3.

WITH subquery AS (
    SELECT 
    id_vendedor,
    DATE(fecha_venta) AS fecha_dia, 
    DENSE_RANK() OVER (PARTITION BY id_vendedor ORDER BY DATE(fecha_venta)) AS rank_fechas
    FROM ventas
), subquery2 AS (
    SELECT 
    id_vendedor, fecha_dia,
    fecha_dia - INTERVAL '1' DAY * rank_fechas AS grupo_fechas
    FROM subquery
)
SELECT id_vendedor, MIN(fecha_dia), MAX(fecha_dia) 
FROM subquery2 GROUP BY id_vendedor, grupo_fechas HAVING COUNT(*) >= 3;




/*
 * PASO 1: Obtener los DÍAS ÚNICOS de venta por vendedor.
 * (Esto evita el problema de ventas duplicadas en un día).
 */
WITH ventas_por_dia AS (
  SELECT DISTINCT
    id_vendedor,
    DATE(fecha_venta) AS fecha_dia
  FROM ventas
),

/*
 * PASO 2: Rankear los días únicos (tu 'subquery').
 */
fechas_con_rango AS (
  SELECT
    id_vendedor,
    fecha_dia,
    DENSE_RANK() OVER (PARTITION BY id_vendedor ORDER BY fecha_dia) AS rank_fechas
  FROM ventas_por_dia
),

/*
 * PASO 3: Crear los grupos de racha (tu 'subquery2').
 */
grupos_de_racha AS (
  SELECT
    id_vendedor,
    fecha_dia,
    (fecha_dia - INTERVAL '1' DAY * rank_fechas) AS grupo_racha
  FROM fechas_con_rango
)

/*
 * PASO 4: Agrupar y filtrar.
 */
SELECT
  id_vendedor,
  MIN(fecha_dia) AS dia_inicio_racha,
  MAX(fecha_dia) AS dia_fin_racha
FROM grupos_de_racha
GROUP BY id_vendedor, grupo_racha
HAVING COUNT(*) >= 3; -- Filtrar rachas de 3+ días