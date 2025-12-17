| ID_Categoria (PK) | Categoria_Nombre |
| :---------------- | :--------------- |
| 1                 | Electrónicos     |
| 2                 | Ropa             |



| ID_Subcategoria (PK) | Subcategoria_Nombre | ID_Categoria (FK) |
| :------------------- | :------------------ | :---------------- |
| 11                   | Televisores         | 1                 |
| 12                   | Laptops             | 1                 |
| 13                   | Audio               | 1                 |
| 21                   | Playeras            | 2                 |



| ID_Producto (PK) | Nombre_Producto   | ID_Subcategoria (FK) |
| :--------------- | :---------------- | :------------------- |
| 101              | TV 55"            | 11                   |
| 102              | TV 65"            | 11                   |
| 103              | Laptop Gamer      | 12                   |
| 104              | Laptop Oficina    | 12                   |
| 105              | Audífonos Pro     | 13                   |
| 106              | Audífonos Sport   | 13                   |
| 201              | Playera Lisa      | 21                   |
| 202              | Playera Estampada | 21                   |