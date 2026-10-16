# Handoff TASK-0033 → TASK-0032 (núcleo Tauri chat)

Contratos acordados en `packages/contracts/src/ipc.ts` y `product.ts`. El renderer invoca estos **nombres de comando** (string literal Tauri = clave del enum):

| Comando | Request | Response `data` |
| --- | --- | --- |
| `list_models` | `ListModelsRequest` | `ListModelsResponse` |
| `chat_create` | `ChatCreateRequest` | `ChatCreateResponse` |
| `chat_list` | `ChatListRequest` | `ChatListResponse` |
| `chat_send` | `ChatSendRequest` | `ChatSendResponse` |

Envelope: `IpcResult<T>` (`{ ok, data? \| error? }`). Payload anidado en `{ request }` (mismo patrón que TASK-0027/0028).

## Comportamiento esperado (0032)

- **`list_models`:** delegar en `ModelRouter`; sin HTTP desde el renderer.
- **`chat_create`:** `notebook_id` obligatorio si `mode === "notebook"`; omitido/null en `general` (R-PROD-007).
- **`chat_list`:** filtrar por `mode` y/o `notebook_id`; devolver mensajes persistidos en `messages_by_chat`.
- **`chat_send`:** si falta `chat_id`, crear chat; si `chat_id` existe y `chat.mode !== request.mode`, rechazar (`VALIDATION_FAILED`); notebook → retrieval FTS + router; general → **cero** FTS; validar citas antes de responder.

## Huellas SHA-256 (TASK-0033, base `15eba56`)

Verificar al integrar en TASK-0032:

| Archivo | SHA-256 |
| --- | --- |
| `packages/contracts/src/ipc.ts` | `E81B42BA9BA651FBEE7923761B29D0D98E69456637B880DC03FB8808BF332FEC` |
| `packages/contracts/src/product.ts` | `4A555536423969FB715525C08D5D4CD0CFD4A18A4FE6C90A70069116489B4BD1` |

## Mock UI (0033)

Hasta que existan handlers Tauri, `apps/desktop/src/lib/ipc-client.ts` degrada a `mock-workspace.ts` si `invoke` falla (igual que import/búsqueda en 0028).
