# SAGA - Web de alquiler de maquinaria - Proyecto Ingeniería de Software II

## Instalación
1. Clonar el repositorio:

```bash
git clone <URL_DEL_REPOSITORIO>
cd <NOMBRE_DEL_REPOSITORIO>
```

2. Crear archivos .env para `backend/`, `frontend/` y `serverMp/` a partir de los `.env.example` de cada directorio mencionado:

```bash
cd <DIRECTORIO>
cp .env.example .env
```

Este comando se debe ejecutar sobre cada uno de los directorios mencionados. En los correspondientes `.env.example` se encuentran detalladas las variables de entorno necesarias con una descripción.

3. Crear la base de datos:

Se debe instalar PostgreSQL. Luego, ejecutar:

```bash
cd backend/
sudo -u postgres psql
CREATE DATABASE saga;
\c saga
```

De esta forma, se ingresa con permisos de root user y se crea la base de datos. 
Se debe indicar en `backend/.env` las URL de las bases de datos creadas (Producción y Testing), según corresponda.

Si ya se tiene un usuario de PostgreSQL configurado con acceso, podes usar:

```bash
psql -U tu_usuario -d postgres
```

Luego, en la consola interactiva de PostgreSQL se ejecuta

```bash
\i createdb.sql
```

Opcionalmente puede ejecutarse, también dentro de la consola interactiva de PostgreSQL, el siguiente comando para introducir datos de prueba a la base de datos.

```bash
\i populate_rows.sql
```

## Ejecución

1. Ejecutar nginx. Las imágenes deben encontrarse en `backend/media/machines/`

```bash
cd backend/
sudo nginx -p ./ -c media/nginx.conf
```

2. Ejecutar el backend. Se elige `test` o `prod` en función de la base de datos que se utilizará.

```bash
cd backend/
cargo run -- <test-prod>
```

3. Ejecutar el frontend.

```bash
cd frontend/
npm i --legacy-peer-deps                        # Solo por primera vez.
npm install chart.js react-chartjs-2 --force    # Solo por primera vez.
npm run dev
```

4. Ejecutar servidor para MercadoPago.

```bash
cd serverMp/
npm i   # Solo por primera vez.
node index.js
```

5. Iniciar NGROK

```bash
ngrok http --url=<URL-NGROK-SIN-HTTP//> 5173
```

Accediendo a la URL de NGROK puede empezar a utilizarse el sistema.