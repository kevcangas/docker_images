Tarea: Escribe una consulta que devuelva el nombre de cada departamento y el monto total de ventas generado por los empleados de ese departamento.

Columnas esperadas: nombre_depto, ventas_totales.

SELECT departamentos.nombre_depto, SUM(ventas.monto) AS ventas_totales 
FROM ventas LEFT JOIN empleados ON ventas.id_vendedor = empleados.id_empleado 
LEFT JOIN departamentos ON departamentos.id_departamento = empleados.id_departamento 
GROUP BY departamentos.nombre_depto;