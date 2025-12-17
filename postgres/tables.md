Comandos para crear tablas necesarias en PostgreSQL

```sql
CREATE DATABASE pruebas;

CREATE SCHEMA schema_pruebas;

CREATE TABLE departamentos(
id_departamento INT Primary Key,
nombre_depto VARCHAR
);

CREATE TABLE empleados(
id_empleado INT Primary Key,
nombre VARCHAR,
id_departamento INT references departamentos(id_departamento),
salario DECIMAL,
fecha_contratacion DATE
);

CREATE TABLE ventas(
id_venta INT Primary Key,
id_producto INT,
id_vendedor INT references empleados(id_empleado),
monto DECIMAL,
fecha_venta TIMESTAMP
);

INSERT INTO departamentos
VALUES
(1, 'Ventas'),
(2, 'IT'),
(3, 'Recursos Humanos');

INSERT INTO empleados
VALUES
(101, 'Ana Torres', 1, 55000.00, '2023-01-15'),
(102, 'Carlos Ruiz', 1, 52000.00, '2023-03-20'),
(103, 'Maria Lopez', 1, 53000.00, '2024-02-10'),
(104, 'David Kim', 2, 70000.00, '2022-07-30'),
(105, 'Sofia Chen', 2, 72000.00, '2023-11-01'),
(106, 'Luis Gonzalez', 3, 48000.00, '2024-05-05');

INSERT INTO ventas
VALUES
(1, 5001, 101, 150.00, '2025-01-10 09:30:00'),
(2, 5002, 102, 200.50, '2025-01-12 11:15:00'),
(3, 5001, 101, 150.00, '2025-01-20 14:00:00'),
(4, 5003, 103, 50.75, '2025-02-05 16:22:00'),
(5, 5004, 101, 300.00, '2025-02-15 10:05:00'),
(6, 5002, 102, 200.50, '2025-03-01 12:00:00'),
(7, 5005, 103, 120.00, '2025-04-05 09:00:00'),
(8, 5001, 101, 150.00, '2025-04-10 11:30:00'),
(9, 5006, 102, 75.00, '2025-05-20 15:45:00'),
(10, 5003, 101, 50.75, '2025-06-18 13:10:00'),
(11, 5007, 103, 250.00, '2025-07-07 10:00:00'),
(12, 5002, 102, 200.50, '2025-07-15 11:20:00'),
(13, 5004, 101, 300.00, '2025-08-01 09:15:00'),
(14, 5001, 103, 150.00, '2025-10-10 14:30:00'),
(15, 5005, 102, 120.00, '2025-11-05 16:00:00');
```