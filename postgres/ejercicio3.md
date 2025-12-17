Tarea: Usando una Cláusula WITH (CTE), primero obtén el salario promedio por nombre_depto. Luego, en la consulta principal, selecciona solo aquellos departamentos cuyo salario promedio sea superior a $50,000.

Columnas esperadas: nombre_depto, salario_promedio.

WITH subquery AS (
    SELECT departamentos.nombre_depto, AVG(empleados.salario) AS salario_promedio 
    FROM empleados LEFT JOIN departamentos ON empleados.id_departamento = departamentos.id_departamento 
    GROUP BY departamentos.nombre_depto
) SELECT * FROM subquery WHERE salario_promedio > 50000;