## Modo de uso

Se ejecutan los siguientes comandos en el orden especificado:

1. docker compose up -d postgres-dw

2. docker compose run --rm dbt-runner debug

3. docker compose run --rm dbt-runner run

4. docker compose exec postgres-dw psql -U admin -d analytics_db -c "SELECT * FROM ventas_mexico_limpias;"