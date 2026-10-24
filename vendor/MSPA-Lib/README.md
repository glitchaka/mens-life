# MSPA-Lib

**MSPA — Motor de Simulación Psicológica y Cultural de Agentes**  
**Agent Psychology Simulation Middleware**

MSPA-Lib es una biblioteca **C++17** para mantener estado psicológico subjetivo de agentes, interpretar estímulos, formar y revisar creencias, producir appraisal y afecto, conservar memoria y relaciones, aprender expectativas y elegir entre acciones que el host declara físicamente disponibles.

> **MSPA procesa significado psicológico; el host controla la realidad y la ejecución.**

Este README es **autocontenido**: contiene el contrato técnico necesario para integrar, mantener y extender MSPA. Los archivos de `docs/` conservan la misma información separada por área para revisión normativa, pero no son capítulos obligatorios para entender la biblioteca.

## Estado

La implementación **v1.0.0** cubre el alcance definido para el núcleo de MSPA:

- runtime multi-instancia;
- lifecycle de agentes;
- semántica estructurada y `RegistrySnapshot`;
- Domain Packs versionados y con dependencias;
- inferencia declarativa acotada;
- personalidad, valores, creencias, appraisal, emociones, ánimo, metas, memoria y relaciones;
- aprendizaje causal contextual;
- Utility AI psicológica e histéresis;
- `OBSERVE`, `DECIDE`, `REPORT`, `ADVANCE_TO`, `REACT` y sus formas batch;
- export/import entre dominios temporales;
- serialización binaria versionada y migración de estado;
- RNG determinista por agente;
- validación defensiva y cuotas;
- diagnostics explicables;
- API C++ y ABI C;
- build estático/compartido, instalación CMake, tests, benchmark y harness de fuzzing.

Lo que **no pertenece al alcance de MSPA** es intencional: física, pathfinding, animación, inventario, combate, percepción física, persistencia del mundo, scheduling del host y adaptadores específicos de juegos/motores. Esas capas viven fuera de este repositorio.

---

# 1. Modelo de integración

```text
HOST / WORLD
    │
    │ OBSERVE: estímulos percibidos y proposiciones
    ▼
MSPA
    ├─ creencias / inferencia
    ├─ appraisal / emoción / mood
    ├─ metas / memoria / relaciones
    └─ aprendizaje
    │
    │ DECIDE: opciones reales ofrecidas por el host
    ▼
INTENCIÓN / ELECCIÓN / EXPLICACIÓN
    │
    │ el host ejecuta la acción real
    ▼
HOST
    │
    │ REPORT: resultado real
    ▼
MSPA actualiza expectativas y estado
```

MSPA **nunca crea una acción física que el host no haya ofrecido**. `DECIDE` sólo selecciona entre `ActionOption` recibidos.

## Autoridad del host

El host posee:

- mundo y entidades físicas;
- percepción y línea de visión;
- navegación y pathfinding;
- física y colisiones;
- inventario, economía y recursos;
- combate y daño físico;
- animación y presentación;
- catálogo de acciones físicamente posibles;
- resultados reales de acciones;
- scheduling y threads del mundo;
- almacenamiento/transportes de bytes;
- semántica específica del consumidor.

## Autoridad de MSPA

MSPA posee:

- personalidad;
- valores;
- creencias subjetivas;
- inferencia declarativa;
- appraisal;
- emociones y ánimo;
- metas;
- memoria;
- relaciones direccionales;
- expectativas aprendidas;
- preferencia entre acciones ofrecidas;
- continuidad de intención;
- trazas explicables.

**El core no contiene nombres, clases, personajes, lugares, repositorios ni reglas de un consumidor concreto.**

---

# 2. Requisitos, build e instalación

Requisitos:

- CMake 3.20+
- C++17
- compilador MSVC, GCC o Clang
- compilador C si se construyen los tests de ABI C

Build estático con tests:

```bash
cmake -S . -B build \
  -DMSPA_BUILD_TESTS=ON \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build -C Release --output-on-failure
```

Build compartido:

```bash
cmake -S . -B build-shared \
  -DMSPA_BUILD_SHARED=ON \
  -DCMAKE_BUILD_TYPE=Release
cmake --build build-shared --config Release
```

Instalación:

```bash
cmake --install build --prefix ./mspa-install
```

Un consumidor CMake puede usar:

```cmake
find_package(MSPA 1 CONFIG REQUIRED)
target_link_libraries(mi_programa PRIVATE MSPA::mspa)
```

Opciones:

| Opción | Default | Función |
|---|---:|---|
| `MSPA_BUILD_TESTS` | `ON` | Construye suites de tests |
| `MSPA_BUILD_BENCHMARKS` | `OFF` | Construye microbenchmark |
| `MSPA_BUILD_SHARED` | `OFF` | Construye biblioteca compartida |
| `MSPA_BUILD_FUZZ` | `OFF` | Construye harness libFuzzer; requiere Clang |

Política numérica de conformidad:

- MSVC: `/fp:strict`;
- GCC/Clang: sin `-ffast-math` y con `-ffp-contract=off`;
- `PsychTime` usa enteros de 64 bits;
- las escalas temporales son racionales, no `float`;
- no se depende del orden de hash ni del scheduling de threads para resultados lógicos.

---

# 3. Estructura del repositorio

```text
include/mspa/
├── mspa.hpp                  API agregada C++
├── c_api.h                   ABI C
├── base/                     IDs, Result, errores, RNG, tiempo, versión
├── model/                    AgentPsychState
├── semantics/                proposiciones, registry, inferencia, Domain Packs
├── psychology/               subsistemas psicológicos
├── decision/                 Utility AI, curvas, consideraciones, histéresis
├── runtime/                  Runtime y AgentStore
├── persistence/              transferencia y serialización
├── validation/               límites y validación
└── diagnostics/              trazas

src/
├── base/
├── semantics/
├── psychology/
├── decision/
├── runtime/
├── persistence/
├── validation/
├── diagnostics/
└── c/

tests/                        suites unitarias/integración/seguridad/C ABI
benchmarks/                   microbenchmark
fuzz/                         harness de deserialización
cmake/                        paquete CMake instalable
docs/                         copias normativas por área
```

`#include <mspa/mspa.hpp>` es el punto de entrada C++.  
`#include <mspa/c_api.h>` es el punto de entrada C.

---

# 4. Identidad y aislamiento

Los identificadores son tipos fuertes (`RuntimeId`, `WorldId`, `NamespaceId`, `ConceptId`, `PredicateId`, `GoalId`, `BeliefId`, `MemoryId`, `ActionTypeId`, `ActionInstanceId`, `ActionContinuityKey`, `PackId`, `TimeDomainId`, etc.).

`AgentId` incluye el mundo:

```text
AgentId
  WorldId world
  uint64 local_id
```

Dos agentes con igual `local_id` y diferente `WorldId` son identidades distintas.

Pueden coexistir varios `Runtime` en el mismo proceso. No existe RNG global, `AgentStore` global ni registry mutable global obligatorio. Cada runtime conserva su propio:

```text
RuntimeConfig
TimeDomain
RegistrySnapshot
AgentStore
SeedRoot
PsychologyEngine
UtilityComposer
TraceSink
RuntimeLimits
```

Un `Runtime` no se considera internamente thread-safe para escrituras concurrentes. El host debe serializar las llamadas que muten **la misma instancia**. Runtimes diferentes pueden administrarse de forma independiente.

---

# 5. Tiempo

```text
PsychTime = uint64 logical ticks
```

Cada runtime fija al crearse:

```text
TimeDomain
  TimeDomainId id
  TimeScale scale

TimeScale
  ticks_per_second_num
  ticks_per_second_den
```

La escala es racional y permanece estable durante la vida del runtime.

## `process_time` y `event_time`

`process_time` indica cuándo MSPA procesa una operación y debe ser monotónico por agente.

`event_time` indica cuándo ocurrió el hecho referido. Puede ser anterior al estado actual, por ejemplo en testimonios tardíos, recuerdos o paquetes de red retrasados.

```text
process_time < LastProcessedTime  -> TimeRegression
event_time < LastProcessedTime    -> permitido
```

---

# 6. Estado de agente

`AgentPsychState` es el estado portable del agente y contiene:

```text
schema_version
AgentId
Personality
Values
Beliefs
Emotions
Mood
Goals
Memories
Relationships
LearnedExpectations
CurrentIntent
RngState
LastProcessedTime
```

## Personalidad

Seis ejes normalizados `[0,1]`:

- sociabilidad;
- estabilidad;
- empatía;
- persistencia;
- curiosidad;
- audacia.

De ellos se derivan sensibilidades como rigidez de creencias, persistencia de metas, resistencia al cambio, sensibilidad a amenaza, sensibilidad social y exploración. La personalidad **modula** cálculos; no se usa como una tabla rígida de “acción + puntos”.

## Valores

`ValueEntry = ConceptId + intensidad`.

Los conceptos concretos se registran externamente. Un Domain Pack puede declarar `initial_values`; un `AgentInit` puede solicitar que se apliquen los defaults del registry mediante `use_registry_default_values`.

---

# 7. Proposiciones y semántica

MSPA no almacena conocimiento como texto libre. Una `Proposition` contiene:

- `PredicateId`;
- argumentos por rol;
- polaridad;
- namespace;
- calificadores;
- tiempo opcional;
- procedencia.

Un argumento puede conservar valor textual, referencia de agente o referencia conceptual.

Un `PredicateSchema` declara:

- roles obligatorios y opcionales;
- rol de agencia;
- roles objetivo;
- categorías de consecuencia;
- pistas de appraisal;
- conceptos relacionados;
- relevancia base;
- consecuencia base;
- intencionalidad.

`validate_proposition` rechaza IDs inválidos, namespaces incompatibles, roles obligatorios ausentes, roles duplicados y entradas que superen cuotas.

---

# 8. Registry y snapshots

`RegistrySnapshot` es inmutable desde la perspectiva de una operación/batch. Contiene:

- predicados;
- conceptos;
- tipos y schemas de acciones;
- consideraciones configurables;
- reglas de inferencia;
- packs cargados y sus versiones;
- valores iniciales declarados.

Cambios semánticos se realizan con `RegistryBuilder`:

```text
snapshot activo
   ↓ copiar
staging RegistryBuilder
   ↓ validar / añadir packs
publish()
   ↓
nuevo snapshot
```

Un batch iniciado sigue usando el snapshot capturado al comienzo. Los cambios se publican entre batches.

---

# 9. Domain Packs

MSPA define el contrato; **los packs concretos pertenecen a consumidores externos**.

`DomainPack` puede declarar:

- ID, namespace y versión;
- versión mínima de MSPA;
- dependencias de otros packs con versión mínima;
- predicados y conceptos;
- tipos y schemas de acción;
- consideraciones y curvas;
- reglas de inferencia;
- `ContextTags` y `OutcomeTags`;
- valores iniciales.

El runtime valida dependencias antes de stagear un pack. No basta con que un `PackId` exista: debe cumplir la versión mínima declarada.

Un pack no puede:

- ejecutar código nativo arbitrario;
- acceder a red/filesystem desde el motor de reglas;
- elevar límites del runtime;
- introducir clases del host dentro del core;
- forzar una emoción universal como verdad física.

---

# 10. Inferencia

`InferenceEngine` implementa forward chaining declarativo acotado.

Una regla declara:

```text
premisas (predicado + polaridad)
→ conclusión (predicado + polaridad + namespace)
+ certeza base
```

La inferencia:

- respeta `max_rules`;
- respeta `max_hypotheses`;
- respeta `max_depth`;
- evita duplicados/ciclos lógicos simples;
- produce hipótesis, no “verdades globales”;
- puede usar reglas registradas por Domain Packs.

No es Turing-completa y no ejecuta callbacks nativos de packs.

---

# 11. Creencias

Una creencia conserva:

- proposición completa;
- certeza;
- carga emocional;
- fuente (`Observation`, `Testimony`, `Inference`, `Memory`, `Influence`, `Learning`);
- agente fuente opcional;
- conteo de evidencia;
- timestamp de actualización.

La actualización por evidencia considera:

- certeza previa;
- certeza percibida;
- confianza en fuente;
- rigidez derivada de personalidad;
- resistencia emocional;
- contradicción o compatibilidad.

La evidencia contradictoria puede debilitar una creencia y, si supera el soporte previo, invertir su polaridad en vez de crear automáticamente “verdad global”.

---

# 12. Appraisal, emociones y ánimo

`AppraisalEngine` produce un vector con:

- relevancia;
- consecuencia;
- agencia;
- intencionalidad;
- certeza;
- control;
- alineación normativa;
- error de expectativa;
- relación.

Usa el schema semántico, valores/metas relacionados, personalidad, control percibido, certeza y relaciones identificables en la proposición.

Emociones básicas implementadas:

- `Joy`;
- `Sadness`;
- `Fear`;
- `Anger`;
- `Anxiety`;
- `Aversion`.

`EmotionSystem` fusiona activaciones compatibles y aplica decaimiento temporal. `MoodSystem` mantiene valencia/activación, absorbe afecto y decae de manera perezosa con `ADVANCE_TO`.

---

# 13. Metas

Las metas conservan concepto, prioridad, compromiso, estado y timestamps.

Estados:

- `Active`;
- `Suspended`;
- `Blocked`;
- `Completed`;
- `Abandoned`.

`GoalSystem` puede:

- crear metas;
- cambiar estado;
- derivar metas desde señales genéricas;
- limitar cuántas dominan simultáneamente;
- aplicar persistencia/decaimiento según personalidad y paso del tiempo.

MSPA no presupone de dónde provino la señal: necesidad física, compromiso, valor, evento o influencia son interpretaciones del host/datos.

---

# 14. Memoria y reapreciación

Una memoria conserva:

- proposición;
- importancia;
- carga emocional;
- certeza;
- fuente;
- participantes;
- `event_time`;
- última reactivación;
- número de reactivaciones.

`MemorySystem` permite recordar, reactivar, buscar, compactar y consolidar. La consolidación prioriza importancia, carga emocional, reactivación y recencia bajo las cuotas del runtime.

`ReappraisalSystem` puede modificar interpretación/carga afectiva de memorias o creencias sin reescribir silenciosamente la proposición factual almacenada.

---

# 15. Relaciones

Las relaciones son **direccionales**:

```text
A -> B != B -> A
```

Cada `RelationshipState` mantiene:

- afecto;
- confianza;
- respeto;
- familiaridad.

`REPORT` puede aplicar `RelationshipImpact` después de una acción real.

---

# 16. Aprendizaje

El aprendizaje causal inicial es deliberadamente acotado:

```text
ActionType + ContextTags -> OutcomeTag
```

Cada patrón conserva:

- intentos;
- éxitos/ocurrencias;
- confianza;
- última activación.

Los tags de contexto se canonicalizan para conservar determinismo. `DECIDE` puede incorporar la expectativa aprendida relevante para la opción actual.

---

# 17. Decisión

`ActionOption` describe sólo opciones físicamente posibles y puede incluir:

- tipo, instancia y `ActionContinuityKey`;
- actor y objetivos;
- coste y riesgo;
- progreso;
- novedad;
- exposición social;
- coste de interrupción;
- estabilidad de contexto;
- `ContextTags`;
- efectos esperados;
- señales nombradas.

`DecisionContext` permite al host proporcionar presiones genéricas, por ejemplo necesidades, **sin hardcodear hambre/sed/etc. en MSPA**:

```text
NeedSignal
  ConceptId
  pressure
```

Los `ExpectedEffect` pueden afectar canales:

- Need;
- Value;
- Goal;
- Relationship;
- Emotion;
- Generic.

`UtilityComposer` combina sólo consideraciones relevantes:

- coste/riesgo/progreso;
- personalidad;
- miedo/ansiedad;
- necesidades;
- valores;
- metas;
- relaciones;
- mood/emociones;
- expectativas aprendidas;
- consideraciones configuradas por Domain Pack.

Las curvas configurables son lineales por tramos y serializables como datos.

## Histéresis

`HysteresisPolicy` usa:

- `ActionContinuityKey`;
- persistencia;
- progreso;
- coste de interrupción;
- estabilidad de contexto;
- urgencia.

Esto evita que pequeñas diferencias de utilidad hagan oscilar la intención en cada evaluación.

---

# 18. Flujo público

## `CREATE_AGENT`

Crea estado psicológico dentro del mundo del runtime. Valida cuotas, personalidad, estado inicial y RNG.

`DerivedInit` permite mezcla determinista de fuentes sin asumir parentesco o biología.

## `REMOVE_AGENT`

`ForgetRuntimeOnly` elimina el estado activo.  
`ValidateNoReferences` rechaza la eliminación si otros estados psicológicos todavía referencian al agente.

## `HAS_AGENT`

Consulta read-only.

## `OBSERVE`

Entrada principal:

```text
AgentId
Proposition
StimulusTime(process_time, event_time?)
perceived_certainty
source_trust
BeliefSource
source_agent?
```

Pipeline:

```text
validación
→ actualización de creencia
→ inferencia acotada
→ appraisal
→ emoción
→ mood
→ memoria
→ ObserveResult
```

`ObserveResult` incluye:

- `state_changed`;
- `decision_recommended`;
- `urgency` (`None`, `Normal`, `Soon`, `Immediate`);
- `current_action_threatened`;
- token diagnóstico.

## `DECIDE`

Recibe acciones disponibles, contexto psicológico externo opcional y continuidad actual. Devuelve selección, objetivo, confianza y traza de candidatos.

## `REPORT`

Comunica el resultado **real** de la acción. Puede incluir:

- éxito/fallo;
- ContextTags;
- OutcomeTags;
- proposiciones resultantes;
- impactos relacionales.

Actualiza aprendizaje y estado asociado.

## `ADVANCE_TO`

Materializa trabajo temporal perezoso: decaimiento emocional/mood, persistencia de metas y consolidación de memoria.

## `REACT`

Wrapper `OBSERVE + DECIDE`; no añade una semántica diferente.

---

# 19. Batch y determinismo

Existen:

- `observe_batch`;
- `decide_batch`;
- `report_batch`;
- `advance_batch`.

El orden canónico es:

```text
(AgentId, input_sequence)
```

Para operaciones repetidas del mismo agente se valida que los tiempos de procesamiento sean compatibles con ese orden. Duplicar `input_sequence` o producir un orden temporal irresoluble devuelve `BatchConflict`.

Una llamada individual es conceptualmente un batch de uno.

El objetivo de determinismo es:

```text
mismo estado + registry + inputs + timestamps + seed
→ mismo resultado lógico
```

El proyecto no promete todavía identidad bit-a-bit entre toolchains/arquitecturas distintas, pero evita deliberadamente fuentes obvias de nondeterminismo lógico.

---

# 20. RNG

Cada agente tiene un stream independiente derivado de:

```text
SeedRoot + AgentId + stream_kind
```

Crear/procesar otro agente no consume su secuencia.

En importación:

- `RekeyForDestination` es el default para traslado;
- `PreserveState` se usa explícitamente para restore/replay del mismo contexto lógico.

---

# 21. Export, import y dominios temporales

`ExportedAgent` contiene:

- estado psicológico;
- dominio temporal de origen;
- `exported_at`.

Políticas de importación:

- `RebaseRelative` — default;
- `PreserveAbsolute` — requiere dominio y escala compatibles;
- `RejectIncompatible`.

`RebaseRelative` convierte y reancla todos los timestamps relevantes:

- `last_processed_time`;
- mood;
- creencias y tiempo de proposiciones;
- emociones;
- metas;
- memorias y tiempo de proposiciones;
- expectativas aprendidas.

Nunca se reinterpretan ticks de un dominio como si fueran ticks de otro.

---

# 22. Serialización y migración

MSPA incluye un formato binario propio para `ExportedAgent`:

- magic `MSPA`;
- versión de formato;
- versión de schema de estado;
- enteros explícitos;
- floats codificados de forma estable;
- longitudes validadas;
- límites de bytes;
- rechazo de truncado, overflow lógico y payload hostil.

APIs C++:

```cpp
serialize_agent(...)
deserialize_agent(...)
migrate_agent_state(...)
Runtime::export_agent_bytes(...)
Runtime::import_agent_bytes(...)
```

El host sigue siendo responsable de **dónde** guarda o transporta esos bytes.

---

# 23. Validación y cuotas

Toda entrada externa se considera no confiable.

Jerarquía:

```text
HardLimits
  ↓
RuntimeLimits
  ↓
DefaultLimits
  ↓
recomendaciones externas
```

Se limitan, entre otros:

- agentes;
- valores;
- creencias;
- emociones;
- memorias;
- metas;
- relaciones;
- patrones de aprendizaje;
- argumentos de proposición;
- tamaño de strings;
- tamaño serializado;
- trazas diagnósticas.

Un Domain Pack no puede elevar `RuntimeLimits` ni `HardLimits`.

---

# 24. Errores

La API C++ usa `Result<T>`.

`ErrorCode` incluye:

- `InvalidArgument`;
- `UnknownAgent`;
- `DuplicateAgent`;
- `InvalidState`;
- `InvalidPack`;
- `QuotaExceeded`;
- `VersionMismatch`;
- `NamespaceConflict`;
- `TimeRegression`;
- `BatchConflict`;
- `OutOfMemory`;
- `UnsupportedOperation`;
- `SerializationError`;
- `InternalInvariantViolation`.

Las fronteras públicas del runtime convierten `std::bad_alloc` en `OutOfMemory` y no dejan escapar excepciones.

La ABI C traduce estos errores a `mspa_status`; `mspa_last_error()` expone el detalle textual del último error del thread.

---

# 25. Diagnostics

`TraceSink` conserva trazas acotadas con token estable para la sesión del runtime.

La ruta explicativa puede registrar:

```text
OBSERVE
→ belief update
→ inference
→ appraisal
→ emotion/mood
→ memory
→ DECIDE candidates
→ utility considerations
→ selected action
→ REPORT
→ learning
```

La instrumentación no consume el RNG del agente ni modifica el resultado lógico.

---

# 26. API C++ mínima

```cpp
#include <mspa/mspa.hpp>

mspa::RuntimeConfig cfg;
cfg.runtime_id = mspa::RuntimeId{1};
cfg.world_id = mspa::WorldId{1};
cfg.time_domain = {
    mspa::TimeDomainId{1},
    mspa::TimeScale{60, 1}
};
cfg.seed_root = 1234;

auto runtime_result = mspa::Runtime::create(cfg);
if (!runtime_result) {
    // runtime_result.error_if()
    return;
}
auto runtime = std::move(*runtime_result.value_if());

mspa::AgentId agent{cfg.world_id, 1};
mspa::AgentInit init;
runtime->create_agent(agent, init, mspa::PsychTime{0});

mspa::ObserveInput obs;
obs.agent = agent;
obs.proposition.predicate = mspa::PredicateId{1};
obs.proposition.name_space = mspa::NamespaceId{1};
obs.time.process_time = mspa::PsychTime{10};
// El PredicateSchema correspondiente debe estar registrado.

auto observed = runtime->observe(obs);
```

Para semántica propia, construye un `RegistryBuilder`, aplica uno o más `DomainPack`, publica el snapshot y crea el runtime con ese snapshot; o usa staging/publicación en un runtime existente.

---

# 27. ABI C

La ABI C está en `mspa/c_api.h` y utiliza un `mspa_runtime*` opaco.

Incluye:

- versión;
- creación/destrucción de runtime;
- crear/eliminar/consultar agente;
- observe;
- decide / decide_ex;
- report / report_ex;
- advance;
- export/import de bytes;
- liberación de buffers;
- último error.

`mspa_decide_ex` acepta necesidades y señales genéricas. Las `mspa_action_option` aceptan tags, efectos esperados y señales por opción.

El header se compila como **C real**, no sólo bajo `extern "C"` desde C++.

---

# 28. Tests, sanitizers, fuzz y benchmark

Suites de CTest:

```text
mspa_core_tests
mspa_semantics_tests
mspa_psychology_tests
mspa_runtime_tests
mspa_persistence_tests
mspa_c_api_tests
mspa_security_tests
mspa_c_header_tests
```

La CI debe validar:

- GCC/Linux Release;
- Clang/Linux con AddressSanitizer + UndefinedBehaviorSanitizer;
- MSVC/Windows Release;
- build compartido Linux.

Fuzzer de deserialización:

```bash
cmake -S . -B build-fuzz \
  -DMSPA_BUILD_TESTS=OFF \
  -DMSPA_BUILD_FUZZ=ON \
  -DCMAKE_CXX_COMPILER=clang++
cmake --build build-fuzz
```

Benchmark:

```bash
cmake -S . -B build-bench -DMSPA_BUILD_BENCHMARKS=ON
cmake --build build-bench --config Release
./build-bench/mspa_benchmark
```

---

# 29. Invariantes de mantenimiento

1. El core permanece agnóstico al host.
2. El host conserva autoridad sobre realidad/ejecución.
3. MSPA sólo elige acciones ofrecidas.
4. Los IDs fuertes no se intercambian implícitamente.
5. `process_time` es monotónico por agente.
6. Registry/configuración visibles son inmutables durante un batch.
7. RNG es independiente por agente.
8. Ninguna excepción cruza las fronteras públicas del runtime o C ABI.
9. Toda entrada externa se valida bajo cuotas.
10. La instrumentación no altera resultados.
11. Packs concretos y adaptadores específicos viven fuera de MSPA.
12. Nuevas reglas psicológicas deben conservar trazabilidad y tests de conformidad.

---

# 30. Documentos normativos

Los siguientes archivos se mantienen **en sus nombres actuales** y están sincronizados con esta implementación:

- `docs/MSPA-SPEC-v2.4.md`
- `docs/MSPA-INTERACTION-CONTRACT-v0.2.md`
- `docs/MSPA-DOMAIN-PACK-CONTRACT-v0.1.md`
- `docs/MSPA-CODE-ARCHITECTURE-v0.1.md`

El README es la entrada técnica autocontenida. Si alguna copia normativa diverge del código/README, la divergencia es un defecto que debe corregirse; no autoriza inventar comportamiento.
