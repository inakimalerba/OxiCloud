# Sistema de Migraciones de Base de Datos

Este documento describe el sistema de migraciones de base de datos implementado en OxiCloud para gestionar cambios de esquema de forma controlada y segura.

## Descripción General

OxiCloud utiliza un sistema de migraciones basado en archivos SQL versionados para garantizar que los cambios en la estructura de la base de datos sean:

- Versionados y rastreables
- Aplicados de forma consistente en todos los entornos
- Reproducibles y comprobables
- Independientes del código de la aplicación

## Estructura de Directorios

```
OxiCloud/
├── migrations/                # Directorio principal de migraciones 
│   ├── 20250408000000_initial_schema.sql   # Migración 1: Esquema inicial
│   ├── 20250408000001_default_users.sql    # Migración 2: Usuarios por defecto
│   └── ...                    # Futuras migraciones
├── src/
    ├── bin/
    │   └── migrate.rs         # Herramienta CLI para ejecutar migraciones
```

## Convenciones de Nomenclatura

Las migraciones siguen el formato: `YYYYMMDDHHMMSS_descripción_breve.sql`, donde:

- `YYYYMMDDHHMMSS`: Timestamp que garantiza el orden correcto (año, mes, día, hora, minuto, segundo)
- `descripción_breve`: Descripción concisa del propósito de la migración
- `.sql`: Extensión de archivo SQL

## Ejecución de Migraciones

Las migraciones se ejecutan mediante una herramienta CLI dedicada:

```bash
cargo run --bin migrate --features migrations
```

Este comando:
1. Conecta con la base de datos configurada en el entorno
2. Busca migraciones en el directorio `/migrations/`
3. Compara las migraciones aplicadas con las disponibles
4. Ejecuta secuencialmente las migraciones pendientes
5. Registra las migraciones aplicadas en una tabla de control

## Creación de Nuevas Migraciones

Para crear una nueva migración:

1. Crea un nuevo archivo en el directorio `migrations/` siguiendo la convención de nomenclatura
2. Define los cambios SQL en el archivo
3. Asegúrate de que los cambios sean compatibles con la versión actual del esquema
4. Ejecuta las migraciones con el comando correspondiente

Ejemplo de estructura para una nueva migración:

```sql
-- Migración: Añadir tabla de etiquetas
-- Descripción: Crea la tabla para almacenar etiquetas de archivos y sus relaciones

-- Crear tabla de etiquetas
CREATE TABLE IF NOT EXISTS auth.tags (
    id SERIAL PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    color TEXT NOT NULL DEFAULT '#3498db',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, name)
);

-- Crear índices
CREATE INDEX IF NOT EXISTS idx_tags_user_id ON auth.tags(user_id);

-- Tabla de relación entre archivos y etiquetas
CREATE TABLE IF NOT EXISTS auth.file_tags (
    id SERIAL PRIMARY KEY,
    tag_id INTEGER NOT NULL REFERENCES auth.tags(id) ON DELETE CASCADE,
    file_id TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tag_id, file_id)
);

-- Comentarios de documentación
COMMENT ON TABLE auth.tags IS 'Almacena etiquetas definidas por usuarios';
COMMENT ON TABLE auth.file_tags IS 'Relación muchos-a-muchos entre archivos y etiquetas';
```

## Guía de Buenas Prácticas

1. **Migraciones Incrementales**: Cada migración debe representar un cambio atómico y coherente.

2. **Migraciones Idempotentes**: Cuando sea posible, usa comandos que pueden ejecutarse múltiples veces sin errores (ej. `CREATE TABLE IF NOT EXISTS`).

3. **Migraciones Forward-Only**: Diseña las migraciones para avanzar, no para revertir. Si necesitas deshacer un cambio, crea una nueva migración.

4. **Compatibilidad Hacia Adelante**: Las migraciones deben ser compatibles con el código existente y el que se va a desplegar.

5. **Prueba Antes de Desplegar**: Prueba las migraciones en un entorno similar al de producción antes de aplicarlas.

6. **Documentación**: Documenta el propósito y los cambios clave de cada migración con comentarios dentro del archivo SQL.

## Solución de Problemas

### Verificación del Estado de las Migraciones

Para verificar qué migraciones se han aplicado, OxiCloud incluye detección en tiempo de inicio:

```rust
// Desde src/common/db.rs
let migration_check = sqlx::query("SELECT EXISTS (SELECT 1 FROM pg_tables WHERE schemaname = 'auth' AND tablename = 'users')")
    .fetch_one(&pool)
    .await;
    
match migration_check {
    Ok(row) => {
        let tables_exist: bool = row.get(0);
        if !tables_exist {
            tracing::warn!("Las tablas de la base de datos no existen. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
        }
    },
    Err(_) => {
        tracing::warn!("No se pudo verificar el estado de las migraciones. Por favor, ejecuta las migraciones con: cargo run --bin migrate --features migrations");
    }
}
```

### Problemas Comunes

1. **Error de conexión a la base de datos**: Verifica la URL de conexión en la variable de entorno `DATABASE_URL`.

2. **Conflictos de migración**: Si una migración falla, revisa los mensajes de error para identificar conflictos con el esquema existente.

3. **Permisos insuficientes**: Asegúrate de que el usuario de la base de datos tenga permisos suficientes para crear esquemas, tablas e índices.

4. **Error "Admin already exists"**: Si al intentar registrar un usuario admin recibes el error "El usuario 'admin' ya existe", sigue estos pasos:

   a. Conéctate al contenedor de PostgreSQL:
   ```bash
   # Encuentra el contenedor
   docker ps
   # Ejemplo: oxicloud-postgres-1
   docker exec -it oxicloud-postgres-1 bash
   ```

   b. Conéctate a la base de datos:
   ```bash
   psql -U postgres -d oxicloud
   ```

   c. Establece el esquema y borra el usuario admin existente:
   ```sql
   SET search_path TO auth;
   DELETE FROM auth.users WHERE username = 'admin';
   ```

   d. Verifica la eliminación:
   ```sql
   SELECT username, email, role FROM auth.users;
   ```

   e. Sal de PostgreSQL:
   ```sql
   \q
   exit
   ```

   f. Ahora puedes registrar un nuevo usuario admin a través de la interfaz de OxiCloud.

   Alternativamente, utiliza el script proporcionado:
   ```bash
   cat scripts/reset_admin.sql | docker exec -i oxicloud-postgres-1 psql -U postgres -d oxicloud
   ```

## Beneficios del Enfoque Basado en Migraciones

- **Separación de Responsabilidades**: Las migraciones están separadas del código de la aplicación.
- **Automatización**: Facilita la automatización de despliegues y CI/CD.
- **Historial de Cambios**: Proporciona un historial claro de cómo ha evolucionado el esquema.
- **Colaboración**: Permite que múltiples desarrolladores contribuyan cambios al esquema de forma ordenada.
- **Entornos Múltiples**: Garantiza que todos los entornos (desarrollo, pruebas, producción) tengan estructuras de base de datos idénticas.