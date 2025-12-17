Tarea: Para cada vendedor, muestra sus ventas totales por mes. Además, en otra columna, muestra las ventas totales que tuvo ese mismo vendedor el mes anterior.

Columnas esperadas: id_vendedor, mes_actual, ventas_mes_actual, ventas_mes_anterior.

Pista: Necesitarás agrupar por vendedor y mes en una CTE, y luego usar la función LAG() en la consulta principal.


WITH subquery AS (
SELECT 
id_vendedor, 
EXTRACT(month FROM fecha_venta) AS mes_actual, 
SUM(monto) AS venta_mes_actual
FROM ventas GROUP BY id_vendedor,EXTRACT(month FROM fecha_venta)
) SELECT 
id_vendedor,
mes_actual,
venta_mes_actual,
LAG(venta_mes_actual,1) OVER (PARTITION BY id_vendedor ORDER BY mes_actual) AS venta_mes_anterior
FROM subquery;


WITH ventas_mensuales AS (
  SELECT
    id_vendedor,
    DATE_TRUNC('month', fecha_venta) AS mes_actual, -- <-- Cambio clave
    SUM(monto) AS venta_mes_actual
  FROM ventas
  GROUP BY id_vendedor, DATE_TRUNC('month', fecha_venta)
)
SELECT
  id_vendedor,
  mes_actual,
  venta_mes_actual,
  LAG(venta_mes_actual, 1) OVER (PARTITION BY id_vendedor ORDER BY mes_actual) AS venta_mes_anterior
FROM ventas_mensuales
ORDER BY id_vendedor, mes_actual;