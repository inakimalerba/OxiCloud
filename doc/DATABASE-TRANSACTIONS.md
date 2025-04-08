# Base de Datos y Transacciones en OxiCloud

## Introducción a Transacciones Explícitas en la Base de Datos

Este documento describe la implementación de transacciones explícitas en OxiCloud para garantizar la integridad de los datos en operaciones de base de datos PostgreSQL.

## ¿Qué son las Transacciones?

Una transacción es una secuencia de operaciones de base de datos tratadas como una única unidad lógica. Las transacciones siguen las propiedades ACID:

- **Atomicidad**: Una transacción es "todo o nada". Si cualquier parte falla, toda la transacción falla.
- **Consistencia**: La base de datos pasa de un estado válido a otro estado válido.
- **Aislamiento**: Las transacciones simultáneas se comportan como si fueran secuenciales.
- **Durabilidad**: Una vez confirmada, la transacción permanece confirmada incluso en caso de fallo del sistema.

## Implementación en OxiCloud

OxiCloud ahora utiliza un enfoque consistente para las transacciones de base de datos mediante la función `with_transaction`, que:

1. Comienza una transacción
2. Ejecuta operaciones
3. Confirma automáticamente si todo fue exitoso
4. Revierte (rollback) automáticamente en caso de error

### Utilidad de Transacciones

En `src/infrastructure/repositories/pg/transaction_utils.rs` hemos implementado:

```rust
/// Helper function to execute database operations in a transaction
pub async fn with_transaction<F, T, E>(
    pool: &Arc<PgPool>,
    operation_name: &str,
    operation: F,
) -> Result<T, E>
where
    F: for<'c> FnOnce(&'c mut Transaction<'_, Postgres>) -> futures::future::BoxFuture<'c, Result<T, E>>,
    E: From<SqlxError> + std::fmt::Display
{ ... }
```

Esta función:
- Recibe un pool de conexiones y un closure con operaciones
- Maneja begin/commit/rollback automáticamente
- Proporciona logging detallado del ciclo de vida de la transacción

### Ejemplo de Uso en Repositorios

```rust
// Creación de un usuario con transacción explícita
async fn create_user(&self, user: User) -> UserRepositoryResult<User> {
    with_transaction(
        &self.pool,
        "create_user",
        |tx| {
            Box::pin(async move {
                // Operación principal - insertar usuario
                sqlx::query("INSERT INTO auth.users ...")
                    .bind(...)
                    .execute(&mut **tx)
                    .await?;
                
                // Operaciones adicionales dentro de la misma transacción
                // ...
                
                Ok(user_clone)
            })
        }
    ).await
}
```

## Casos de Uso Implementados

### En UserPgRepository

1. **Creación de Usuario**
   - Garantiza que todas las operaciones de inserción son atómicas
   - Permite agregar operaciones relacionadas (como configuración de permisos)

2. **Actualización de Usuario**
   - Asegura que las modificaciones se apliquen completamente o no se apliquen en absoluto
   - Soporta operaciones combinadas como actualización de información de perfil y preferencias

### En SessionPgRepository

1. **Creación de Sesión**
   - Inserta la sesión y actualiza el timestamp de último acceso del usuario en una única transacción
   - Garantiza consistencia entre sesiones y datos de usuario

2. **Revocación de Sesiones**
   - Asegura que la revocación de una sesión o de todas las sesiones de un usuario sea atómica
   - Permite registrar eventos de seguridad dentro de la misma transacción

## Niveles de Aislamiento

OxiCloud admite diferentes niveles de aislamiento de transacciones mediante `with_transaction_isolation`:

```rust
// Ejemplo de uso con nivel de aislamiento específico
with_transaction_isolation(
    &pool,
    "operacion_critica",
    sqlx::postgres::PgIsolationLevel::Serializable,
    |tx| { ... }
).await
```

Los niveles de aislamiento disponibles son:

1. **Read Committed** (predeterminado)
   - Garantiza que los datos leídos están confirmados
   - No previene lecturas no repetibles o fantasma

2. **Repeatable Read**
   - Garantiza que las lecturas sean consistentes durante toda la transacción
   - Previene lecturas no repetibles pero no lecturas fantasma

3. **Serializable**
   - Nivel más alto de aislamiento
   - Garantiza que las transacciones se comporten como si se ejecutaran en serie
   - Puede causar errores de serialización que requieren reintento

## Mejores Prácticas

1. **Duración de Transacciones**
   - Mantén las transacciones lo más cortas posible
   - Evita operaciones de larga duración dentro de transacciones

2. **Manejo de Errores**
   - Los errores dentro de una transacción provocan rollback automático
   - Utiliza logging adecuado para diagnosticar fallos

3. **Límites de Transacción**
   - Define claramente dónde comienzan y terminan las transacciones
   - Agrupa operaciones relacionadas en una sola transacción

4. **Aislamiento Apropiado**
   - Usa el nivel de aislamiento más bajo adecuado para tu caso de uso
   - Considera serializable para operaciones críticas con posibilidad de conflicto

## Ventajas de Transacciones Explícitas

1. **Integridad de Datos Mejorada**
   - Garantía ACID para operaciones complejas
   - Prevención de estados inconsistentes

2. **Mejor Manejo de Errores**
   - Rollback automático ante fallos
   - Comportamiento predecible en caso de error

3. **Concurrencia Segura**
   - Manejo adecuado de operaciones simultáneas
   - Prevención de condiciones de carrera

4. **Rendimiento**
   - Reducción de trips a la base de datos
   - Operaciones en lote para mejor eficiencia

## Consideraciones de Rendimiento

- Las transacciones añaden cierta sobrecarga
- El rendimiento puede verse afectado por:
  - Duración de la transacción
  - Nivel de aislamiento
  - Número de registros afectados
  - Contención por bloqueos

## Conclusión

La implementación de transacciones explícitas en OxiCloud mejora significativamente la robustez del sistema y garantiza la integridad de los datos en escenarios complejos. El enfoque modular y la API de transacciones simplificada permiten extender fácilmente estos beneficios a nuevas funcionalidades.