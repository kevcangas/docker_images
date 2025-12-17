Tarea: Encuentra los 5 empleados (vendedores) con el mayor monto total de ventas.

Columnas esperadas: nombre (del empleado), monto_total_vendido.

SELECT empleados.nombre AS nombre, SUM(ventas.monto) AS monto_total_vendido 
FROM ventas LEFT JOIN empleados ON ventas.id_vendedor = empleados.id_empleado 
GROUP BY empleados.nombre 
ORDER BY monto_total_vendido DESC LIMIT 5;