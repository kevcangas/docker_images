Tarea: Escribe una consulta que muestre a todos los empleados, su departamento y su salario. Añade una columna que muestre el "ranking" de su salario (del más alto al más bajo) dentro de su propio departamento.

Columnas esperadas: nombre, nombre_depto, salario, ranking_salario_depto.

Pista: Usa RANK() OVER (PARTITION BY ... ORDER BY ...).



SELECT 
empleados.nombre, 
departamentos.nombre_depto, 
empleados.salario, 
RANK() OVER (PARTITION BY departamentos.id_departamento ORDER BY empleados.salario DESC) AS ranking_salario_depto
FROM empleados LEFT JOIN departamentos ON empleados.id_departamento = departamentos.id_departamento;