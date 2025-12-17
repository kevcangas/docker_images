Tarea: Calcula el total de ventas acumulado día por día para todo el mes de enero de 2025.

Columnas esperadas: fecha_dia, ventas_del_dia, total_acumulado.

Pista: Primero agrupa por día, luego usa SUM(...) OVER (ORDER BY fecha_dia).


SELECT 
DATE(fecha_venta) AS fecha_dia, 
COUNT(id_venta) AS ventas_del_dia, 
SUM(SUM(monto)) OVER (ORDER BY fecha_venta) AS total_acumulado
FROM ventas 
WHERE fecha_venta >= '2025-01-01 00:00:00' AND fecha_venta < '2025-02-01 00:00:00'
GROUP BY DATE(fecha_venta);



/*
 * PASO 1: Agrupa las ventas POR DÍA (usando DATE())
 * para obtener el total de ventas de cada día.
 */
WITH ventas_por_dia AS (
  SELECT
    DATE(fecha_venta) AS fecha_dia,
    SUM(monto) AS ventas_del_dia
  FROM ventas
  WHERE fecha_venta >= '2025-01-01 00:00:00'
    AND fecha_venta < '2025-02-01 00:00:00'
  GROUP BY DATE(fecha_venta)
)
/*
 * PASO 2: Ahora, aplica la función de ventana
 * a esos resultados ya agrupados.
 */
SELECT
  fecha_dia,
  ventas_del_dia,
  SUM(ventas_del_dia) OVER (ORDER BY fecha_dia) AS total_acumulado
FROM ventas_por_dia
ORDER BY fecha_dia;