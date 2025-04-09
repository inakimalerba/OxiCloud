# Solución para el issue #45: [BUG] Admin already exists?

## Descripción del problema

Al intentar configurar un usuario administrador, algunos usuarios reciben el error:

```
2025-04-09T14:01:26.733566Z ERROR oxicloud::interfaces::api::handlers::auth_handler: Registration failed for user admin: Already Exists: El usuario 'admin' ya existe
```

## Causa raíz

Este problema ocurre debido a cómo está implementada la migración de datos inicial. En el archivo `migrations/20250408000001_default_users.sql`, el sistema intenta crear un usuario admin por defecto durante la migración inicial, pero:

1. Si luego el usuario intenta crear manualmente otro usuario con nombre "admin", el sistema detecta el conflicto.
2. La cláusula `ON CONFLICT (id) DO NOTHING` solo previene conflictos en el ID, no en el nombre de usuario.

## Solución implementada

Hemos realizado los siguientes cambios:

1. Modificado `migrations/20250408000001_default_users.sql` para comprobar primero si el usuario "admin" ya existe:

```sql
-- Check if admin user already exists before creating it
DO $$$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM auth.users WHERE username = 'admin') THEN
        -- Create admin user (password: Admin123!)
        INSERT INTO auth.users (
            id, 
            username, 
            ...
        ) VALUES (...);
    END IF;
END;
$$$;
```

2. Creado un script `scripts/reset_admin.sql` que puede ejecutarse para eliminar el usuario admin existente:

```sql
-- Set the correct schema
SET search_path TO auth;

-- Delete the admin user if it exists
DELETE FROM auth.users WHERE username = 'admin';

-- Output remaining users for verification
SELECT username, email, role FROM auth.users ORDER BY role, username;
```

3. Actualizado la documentación en `doc/DATABASE-MIGRATIONS.md` con instrucciones detalladas para resolver este problema.

## Instrucciones para usuarios afectados

Si encuentras el error "Admin already exists", tienes dos opciones:

### Opción 1: Usar el script proporcionado
```bash
cat scripts/reset_admin.sql | docker exec -i oxicloud-postgres-1 psql -U postgres -d oxicloud
```

### Opción 2: Hacerlo manualmente
1. Conéctate al contenedor de PostgreSQL:
```bash
# Encuentra el contenedor
docker ps
# Ejemplo: oxicloud-postgres-1
docker exec -it oxicloud-postgres-1 bash
```

2. Conéctate a la base de datos:
```bash
psql -U postgres -d oxicloud
```

3. Borra el usuario admin existente:
```sql
SET search_path TO auth;
DELETE FROM auth.users WHERE username = 'admin';
```

4. Verifica que se eliminó correctamente:
```sql
SELECT username, email, role FROM auth.users;
```

5. Sal y registra un nuevo usuario admin a través de la interfaz web.
