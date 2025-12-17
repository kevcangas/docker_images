Tarea: Crea un reporte que muestre el total de ventas de cada vendedor, pero con columnas separadas para cada trimestre (Q1, Q2, Q3, Q4) del año 2025.

Columnas esperadas: id_vendedor, ventas_Q1, ventas_Q2, ventas_Q3, ventas_Q4.

SELECT 
id_vendedor,  
SUM(CASE
    WHEN extract(quarter from fecha_venta) = 1 THEN monto
    ELSE 0
END) AS ventas_q1,
SUM(CASE
    WHEN extract(quarter from fecha_venta) = 2 THEN monto
    ELSE 0
END) AS ventas_q2,
SUM(CASE
    WHEN extract(quarter from fecha_venta) = 3 THEN monto
    ELSE 0
END) AS ventas_q3,
SUM(CASE
    WHEN extract(quarter from fecha_venta) = 4 THEN monto
    ELSE 0
END) AS ventas_q4
FROM ventas GROUP BY id_vendedor;


# Sin pivotear
WITH subquery AS (SELECT id_vendedor, extract(quarter from fecha_venta) AS quarter, monto FROM ventas)
SELECT
id_vendedor,
quarter, 
SUM(monto)
FROM subquery
GROUP BY quarter, id_vendedor
ORDER BY id_vendedor, quarter;